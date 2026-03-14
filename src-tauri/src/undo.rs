use std::fs;
use std::path::{Path, PathBuf};

use tauri::Manager;

use crate::models::UndoLog;

fn undo_log_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Cannot resolve app data dir: {}", e))?;
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Cannot create app data dir: {}", e))?;
    Ok(data_dir.join("undo_log.json"))
}

pub fn save_undo_log(app: &tauri::AppHandle, log: &UndoLog) -> Result<(), String> {
    let path = undo_log_path(app)?;
    let json = serde_json::to_string_pretty(log)
        .map_err(|e| format!("Failed to serialize undo log: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("Failed to write undo log: {}", e))
}

pub fn load_undo_log(app: &tauri::AppHandle) -> Result<Option<UndoLog>, String> {
    let path = undo_log_path(app)?;
    if !path.exists() {
        return Ok(None);
    }
    let json = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read undo log: {}", e))?;
    let log: UndoLog = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse undo log: {}", e))?;
    Ok(Some(log))
}

pub fn clear_undo_log(app: &tauri::AppHandle) -> Result<(), String> {
    let path = undo_log_path(app)?;
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to clear undo log: {}", e))?;
    }
    Ok(())
}

pub fn execute_undo(app: &tauri::AppHandle) -> Result<String, String> {
    let log = load_undo_log(app)?
        .ok_or_else(|| "No rename operation to undo".to_string())?;

    let mut success = 0;
    let mut errors = Vec::new();

    for entry in log.entries.iter().rev() {
        let new_path = Path::new(&entry.new_path);
        let original_path = Path::new(&entry.original_path);

        if entry.was_heic_conversion {
            match &entry.backup_original_path {
                Some(backup_path) => {
                    let backup_path = Path::new(backup_path);
                    if !backup_path.exists() {
                        errors.push(format!(
                            "Backup copy not found for {}",
                            entry.original_path
                        ));
                        continue;
                    }

                    if new_path.exists() {
                        if let Err(e) = fs::remove_file(new_path) {
                            errors.push(format!(
                                "Failed to remove converted file {}: {}",
                                new_path.display(),
                                e
                            ));
                            continue;
                        }
                    }

                    if let Err(e) = fs::copy(backup_path, original_path) {
                        errors.push(format!(
                            "Failed to restore original from backup {}: {}",
                            original_path.display(),
                            e
                        ));
                    } else {
                        success += 1;
                    }
                }
                None => {
                    if !new_path.exists() {
                        errors.push(format!("File not found: {}", entry.new_path));
                        continue;
                    }

                    let fallback_jpg_path = original_path.with_extension("jpg");
                    if let Err(e) = fs::rename(new_path, &fallback_jpg_path) {
                        errors.push(format!(
                            "Failed to partially restore converted file {}: {}",
                            new_path.display(),
                            e
                        ));
                    } else {
                        success += 1;
                    }
                }
            }
        } else if !new_path.exists() {
            errors.push(format!("File not found: {}", entry.new_path));
            continue;
        } else if let Err(e) = fs::rename(new_path, original_path) {
            errors.push(format!("Failed to restore {}: {}", new_path.display(), e));
        } else {
            success += 1;
        }
    }

    clear_undo_log(app)?;

    if errors.is_empty() {
        Ok(format!("Successfully restored {} files", success))
    } else {
        Ok(format!(
            "Restored {} files with {} errors: {}",
            success,
            errors.len(),
            errors.join("; ")
        ))
    }
}
