mod backup;
mod commands;
mod exif_handler;
mod heic_converter;
mod models;
mod renamer;
mod undo;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::check_tools,
            commands::scan_files,
            commands::preview_rename,
            commands::execute_rename,
            commands::undo_last_rename,
            commands::has_undo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
