use std::path::Path;

use chrono::NaiveDateTime;
use tauri::Emitter;
use walkdir::WalkDir;

use crate::backup;
use crate::exif_handler;
use crate::heic_converter;
use crate::models::*;
use crate::renamer;
use crate::undo;

#[tauri::command]
pub fn check_tools() -> ToolsStatus {
    ToolsStatus {
        exiftool: exif_handler::is_exiftool_available(),
        heif_convert: heic_converter::is_heif_convert_available(),
    }
}

#[tauri::command]
pub fn scan_files(paths: Vec<String>) -> Result<Vec<FileEntry>, String> {
    let mut entries = Vec::new();

    for path_str in &paths {
        let path = Path::new(path_str);

        if path.is_dir() {
            for dir_entry in WalkDir::new(path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if dir_entry.file_type().is_file() {
                    if let Some(entry) = scan_single_file(dir_entry.path()) {
                        entries.push(entry);
                    }
                }
            }
        } else if path.is_file() {
            if let Some(entry) = scan_single_file(path) {
                entries.push(entry);
            }
        }
    }

    Ok(entries)
}

fn scan_single_file(path: &Path) -> Option<FileEntry> {
    let ext = path.extension()?.to_str()?.to_lowercase();

    if !is_supported_extension(&ext) {
        return None;
    }

    let filename = path.file_name()?.to_string_lossy().to_string();
    let (datetime, date_source) = exif_handler::read_date_from_file(path);

    Some(FileEntry {
        path: path.to_string_lossy().to_string(),
        filename,
        extension: ext.clone(),
        date_source,
        datetime: datetime.map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string()),
        is_heic: is_heic_extension(&ext),
    })
}

#[tauri::command]
pub fn preview_rename(
    entries: Vec<FileEntry>,
    format: RenameFormat,
    offset_seconds: i64,
    convert_heic: bool,
) -> Result<Vec<PreviewEntry>, String> {
    Ok(renamer::generate_previews(
        &entries,
        &format,
        offset_seconds,
        convert_heic,
    ))
}

#[tauri::command]
pub async fn execute_rename(
    app: tauri::AppHandle,
    entries: Vec<FileEntry>,
    format: RenameFormat,
    offset_seconds: i64,
    create_backup: bool,
    convert_heic: bool,
) -> Result<RenameResult, String> {
    let total = entries.len();
    let mut success_count = 0;
    let mut errors: Vec<RenameErrorEntry> = Vec::new();
    let mut undo_entries: Vec<UndoEntry> = Vec::new();

    let previews = renamer::generate_previews(&entries, &format, offset_seconds, convert_heic);

    if create_backup {
        backup::create_backup(&entries)?;
    }

    let has_exiftool = exif_handler::is_exiftool_available();

    for (i, (entry, preview)) in entries.iter().zip(previews.iter()).enumerate() {
        let _ = app.emit(
            "rename-progress",
            ProgressPayload {
                current: i + 1,
                total,
                filename: entry.filename.clone(),
            },
        );

        let original_path = Path::new(&entry.path);
        let parent = original_path.parent().unwrap_or(Path::new("."));
        let new_path = parent.join(&preview.new_name);

        if entry.date_source == DateSource::None {
            errors.push(RenameErrorEntry {
                filename: entry.filename.clone(),
                reason: "No date available, skipped".to_string(),
            });
            continue;
        }

        // Skip if source and target are the same
        if original_path == new_path {
            if offset_seconds != 0 {
                if let Some(ref dt_str) = entry.datetime {
                    if let Ok(dt) = NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%dT%H:%M:%S") {
                        let adjusted = renamer::apply_offset(&dt, offset_seconds);
                        let ft = filetime::FileTime::from_unix_time(adjusted.and_utc().timestamp(), 0);
                        let _ = filetime::set_file_times(original_path, ft, ft);

                        if has_exiftool {
                            let _ = exif_handler::write_exif_dates(original_path, &adjusted);
                        }
                    }
                }
            }
            success_count += 1;
            continue;
        }

        // Ensure target doesn't already exist on disk (outside our batch)
        let final_path = resolve_conflict(&new_path);

        let rename_result = if entry.is_heic && convert_heic {
            heic_converter::convert_heic_to_jpg(original_path, &final_path, 90)
                .and_then(|()| {
                    std::fs::remove_file(original_path)
                        .map_err(|e| format!("Conversion succeeded but failed to remove original: {}", e))
                })
        } else {
            std::fs::rename(original_path, &final_path)
                .map_err(|e| e.to_string())
        };

        match rename_result {
            Ok(()) => {
                if offset_seconds != 0 {
                    if let Some(ref dt_str) = entry.datetime {
                        if let Ok(dt) = NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%dT%H:%M:%S")
                        {
                            let adjusted = renamer::apply_offset(&dt, offset_seconds);
                            let ft = filetime::FileTime::from_unix_time(
                                adjusted.and_utc().timestamp(),
                                0,
                            );
                            let _ = filetime::set_file_times(&final_path, ft, ft);

                            if has_exiftool {
                                let _ = exif_handler::write_exif_dates(&final_path, &adjusted);
                            }
                        }
                    }
                }

                undo_entries.push(UndoEntry {
                    original_path: entry.path.clone(),
                    new_path: final_path.to_string_lossy().to_string(),
                });
                success_count += 1;
            }
            Err(e) => {
                errors.push(RenameErrorEntry {
                    filename: entry.filename.clone(),
                    reason: e,
                });
            }
        }
    }

    let undo_log = UndoLog {
        entries: undo_entries,
    };
    let _ = undo::save_undo_log(&app, &undo_log);

    Ok(RenameResult {
        success_count,
        error_count: errors.len(),
        errors,
    })
}

fn resolve_conflict(target: &Path) -> std::path::PathBuf {
    if !target.exists() {
        return target.to_path_buf();
    }

    let stem = target
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = target
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let parent = target.parent().unwrap_or(Path::new("."));

    for i in 1..=999 {
        let candidate = if ext.is_empty() {
            parent.join(format!("{}_{}", stem, i))
        } else {
            parent.join(format!("{}_{}.{}", stem, i, ext))
        };
        if !candidate.exists() {
            return candidate;
        }
    }

    target.to_path_buf()
}

#[tauri::command]
pub async fn undo_last_rename(app: tauri::AppHandle) -> Result<String, String> {
    undo::execute_undo(&app)
}

#[tauri::command]
pub fn has_undo(app: tauri::AppHandle) -> bool {
    undo::load_undo_log(&app)
        .ok()
        .flatten()
        .map(|log| !log.entries.is_empty())
        .unwrap_or(false)
}
