use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::models::FileEntry;

pub fn create_backup(entries: &[FileEntry]) -> Result<Vec<String>, String> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();

    let mut dirs: HashMap<PathBuf, Vec<&FileEntry>> = HashMap::new();
    for entry in entries {
        let path = Path::new(&entry.path);
        let parent = path
            .parent()
            .unwrap_or(Path::new("."))
            .to_path_buf();
        dirs.entry(parent).or_default().push(entry);
    }

    let mut backup_dirs = Vec::new();

    for (dir, files) in &dirs {
        let backup_dir = dir.join(format!("backup_{}", timestamp));
        std::fs::create_dir_all(&backup_dir)
            .map_err(|e| format!("Failed to create backup directory: {}", e))?;

        for file_entry in files {
            let src = Path::new(&file_entry.path);
            let dst = backup_dir.join(&file_entry.filename);
            std::fs::copy(src, &dst)
                .map_err(|e| format!("Failed to backup {}: {}", file_entry.filename, e))?;
        }

        backup_dirs.push(backup_dir.to_string_lossy().to_string());
    }

    Ok(backup_dirs)
}
