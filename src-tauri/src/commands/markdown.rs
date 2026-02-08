use std::fs;
use tauri_plugin_dialog::{DialogExt, FilePath};

#[tauri::command]
pub async fn read_markdown_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read '{}': {}", path, e))
}

#[tauri::command]
pub async fn open_markdown_dialog(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let file = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown", "mdx"])
        .blocking_pick_file();

    match file {
        Some(file_path) => match file_path {
            FilePath::Path(p) => Ok(Some(p.to_string_lossy().to_string())),
            FilePath::Url(u) => Ok(Some(u.to_string())),
        },
        None => Ok(None),
    }
}
