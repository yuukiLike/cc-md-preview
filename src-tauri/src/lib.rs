mod commands;
mod platform;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::markdown::read_markdown_file,
            commands::markdown::open_markdown_dialog,
            commands::chrome_cache::detect_browsers,
            commands::chrome_cache::get_cache_info,
            commands::chrome_cache::list_cache_entries,
            commands::chrome_cache::clean_cache,
            commands::chrome_cache::get_chrome_profiles,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
