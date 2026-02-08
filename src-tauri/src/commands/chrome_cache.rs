use crate::platform::cache_paths;
use serde::{Deserialize, Serialize};
use std::fs;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheInfo {
    pub profile: String,
    pub categories: Vec<CacheCategory>,
    pub total_size: u64,
    pub total_files: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheCategory {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub file_count: u64,
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheEntry {
    pub name: String,
    pub size: u64,
    pub modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanResult {
    pub deleted_files: u64,
    pub freed_bytes: u64,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserInfo {
    pub name: String,
    pub installed: bool,
}

#[tauri::command]
pub async fn detect_browsers() -> Result<Vec<BrowserInfo>, String> {
    let browsers = cache_paths::detect_installed_browsers();
    Ok(browsers
        .into_iter()
        .map(|(name, installed)| BrowserInfo { name, installed })
        .collect())
}

#[tauri::command]
pub async fn get_cache_info(browser: Option<String>) -> Result<CacheInfo, String> {
    let browser = browser.as_deref().unwrap_or("Chrome");
    let categories_config = cache_paths::get_cache_categories_for_browser(browser, "Default");
    let mut categories = Vec::new();
    let mut total_size: u64 = 0;
    let mut total_files: u64 = 0;

    for (name, path) in categories_config {
        let exists = path.exists();
        let (size, file_count) = if exists {
            calculate_dir_size(&path)
        } else {
            (0, 0)
        };
        total_size += size;
        total_files += file_count;
        categories.push(CacheCategory {
            name,
            path: path.to_string_lossy().to_string(),
            size,
            file_count,
            exists,
        });
    }

    Ok(CacheInfo {
        profile: "Default".to_string(),
        categories,
        total_size,
        total_files,
    })
}

#[tauri::command]
pub async fn list_cache_entries(
    cache_type: String,
    browser: Option<String>,
) -> Result<Vec<CacheEntry>, String> {
    let browser = browser.as_deref().unwrap_or("Chrome");
    let categories = cache_paths::get_cache_categories_for_browser(browser, "Default");
    let path = categories
        .iter()
        .find(|(name, _)| name == &cache_type)
        .map(|(_, p)| p.clone())
        .ok_or_else(|| format!("Unknown cache type: {}", cache_type))?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    let read_dir = fs::read_dir(&path).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in read_dir.flatten() {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                let modified = metadata
                    .modified()
                    .ok()
                    .map(|t| {
                        let datetime: chrono::DateTime<chrono::Utc> = t.into();
                        datetime.to_rfc3339()
                    })
                    .unwrap_or_default();

                entries.push(CacheEntry {
                    name: entry.file_name().to_string_lossy().to_string(),
                    size: metadata.len(),
                    modified,
                });
            }
        }
    }

    entries.sort_by(|a, b| b.size.cmp(&a.size));
    Ok(entries)
}

#[tauri::command]
pub async fn clean_cache(
    cache_types: Vec<String>,
    browser: Option<String>,
) -> Result<CleanResult, String> {
    let browser = browser.as_deref().unwrap_or("Chrome");
    let categories = cache_paths::get_cache_categories_for_browser(browser, "Default");
    let mut deleted_files: u64 = 0;
    let mut freed_bytes: u64 = 0;
    let mut errors = Vec::new();

    for cache_type in &cache_types {
        if let Some((_, path)) = categories.iter().find(|(name, _)| name == cache_type) {
            if path.exists() {
                match fs::read_dir(path) {
                    Ok(read_dir) => {
                        for entry in read_dir.flatten() {
                            let entry_path = entry.path();
                            match fs::metadata(&entry_path) {
                                Ok(meta) => {
                                    let size = if meta.is_file() {
                                        meta.len()
                                    } else {
                                        calculate_dir_size(&entry_path).0
                                    };
                                    let result = if meta.is_file() {
                                        fs::remove_file(&entry_path)
                                    } else {
                                        fs::remove_dir_all(&entry_path)
                                    };
                                    match result {
                                        Ok(_) => {
                                            deleted_files += 1;
                                            freed_bytes += size;
                                        }
                                        Err(e) => {
                                            errors.push(format!(
                                                "{}: {}",
                                                entry_path.display(),
                                                e
                                            ));
                                        }
                                    }
                                }
                                Err(e) => {
                                    errors.push(format!("{}: {}", entry_path.display(), e));
                                }
                            }
                        }
                    }
                    Err(e) => errors.push(format!("{}: {}", path.display(), e)),
                }
            }
        }
    }

    Ok(CleanResult {
        deleted_files,
        freed_bytes,
        errors,
    })
}

#[tauri::command]
pub async fn get_chrome_profiles(browser: Option<String>) -> Result<Vec<String>, String> {
    let browser = browser.as_deref().unwrap_or("Chrome");
    let base = cache_paths::get_browser_base_dir(browser);
    let mut profiles = Vec::new();

    if base.exists() {
        if let Ok(read_dir) = fs::read_dir(&base) {
            for entry in read_dir.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "Default" || name.starts_with("Profile ") {
                    profiles.push(name);
                }
            }
        }
    }

    profiles.sort();
    Ok(profiles)
}

fn calculate_dir_size(path: &std::path::Path) -> (u64, u64) {
    let mut size: u64 = 0;
    let mut count: u64 = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if let Ok(meta) = entry.metadata() {
            if meta.is_file() {
                size += meta.len();
                count += 1;
            }
        }
    }
    (size, count)
}
