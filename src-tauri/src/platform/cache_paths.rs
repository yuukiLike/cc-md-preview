use std::path::PathBuf;

/// Returns (category_name, path) pairs for Chrome cache directories.
pub fn get_cache_categories(profile: &str) -> Vec<(String, PathBuf)> {
    let mut categories = Vec::new();
    let home = dirs::home_dir().unwrap_or_default();

    #[cfg(target_os = "macos")]
    {
        let caches = home.join("Library/Caches/Google/Chrome").join(profile);
        let app_support = home
            .join("Library/Application Support/Google/Chrome")
            .join(profile);

        categories.push(("HTTP Cache".to_string(), caches.join("Cache/Cache_Data")));
        categories.push(("Code Cache".to_string(), caches.join("Code Cache")));
        categories.push(("GPU Cache".to_string(), app_support.join("GPUCache")));
    }

    #[cfg(target_os = "windows")]
    {
        let local_app_data = dirs::data_local_dir().unwrap_or_default();
        let chrome_user_data = local_app_data
            .join("Google\\Chrome\\User Data")
            .join(profile);

        categories.push((
            "HTTP Cache".to_string(),
            chrome_user_data.join("Cache\\Cache_Data"),
        ));
        categories.push((
            "Code Cache".to_string(),
            chrome_user_data.join("Code Cache"),
        ));
        categories.push((
            "GPU Cache".to_string(),
            chrome_user_data.join("GPUCache"),
        ));
    }

    categories
}

/// Returns the Chrome user data base directory (for profile listing).
pub fn get_chrome_base_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_default();

    #[cfg(target_os = "macos")]
    {
        home.join("Library/Caches/Google/Chrome")
    }

    #[cfg(target_os = "windows")]
    {
        let local_app_data = dirs::data_local_dir().unwrap_or_default();
        local_app_data.join("Google\\Chrome\\User Data")
    }
}
