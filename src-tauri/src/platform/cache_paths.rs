use std::path::PathBuf;

/// Returns (category_name, path) pairs for cache directories of a given browser.
pub fn get_cache_categories_for_browser(browser: &str, profile: &str) -> Vec<(String, PathBuf)> {
    let mut categories = Vec::new();
    let home = dirs::home_dir().unwrap_or_default();

    #[cfg(target_os = "macos")]
    {
        let (caches_name, app_support_name) = match browser {
            "Chrome Canary" => ("Google/Chrome Canary", "Google/Chrome Canary"),
            _ => ("Google/Chrome", "Google/Chrome"),
        };

        let caches = home.join("Library/Caches").join(caches_name).join(profile);
        let app_support = home
            .join("Library/Application Support")
            .join(app_support_name)
            .join(profile);

        categories.push(("HTTP Cache".to_string(), caches.join("Cache/Cache_Data")));
        categories.push(("Code Cache".to_string(), caches.join("Code Cache")));
        categories.push(("GPU Cache".to_string(), app_support.join("GPUCache")));
    }

    #[cfg(target_os = "windows")]
    {
        let local_app_data = dirs::data_local_dir().unwrap_or_default();
        let user_data_name = match browser {
            "Chrome Canary" => "Google\\Chrome SxS\\User Data",
            _ => "Google\\Chrome\\User Data",
        };
        let chrome_user_data = local_app_data.join(user_data_name).join(profile);

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

/// Returns the browser user data base directory (for profile listing).
pub fn get_browser_base_dir(browser: &str) -> PathBuf {
    let home = dirs::home_dir().unwrap_or_default();

    #[cfg(target_os = "macos")]
    {
        let name = match browser {
            "Chrome Canary" => "Google/Chrome Canary",
            _ => "Google/Chrome",
        };
        home.join("Library/Caches").join(name)
    }

    #[cfg(target_os = "windows")]
    {
        let local_app_data = dirs::data_local_dir().unwrap_or_default();
        let name = match browser {
            "Chrome Canary" => "Google\\Chrome SxS\\User Data",
            _ => "Google\\Chrome\\User Data",
        };
        local_app_data.join(name)
    }
}

/// Detects which browsers are installed.
pub fn detect_installed_browsers() -> Vec<(String, bool)> {
    let mut browsers = Vec::new();

    let chrome_exists = get_browser_base_dir("Chrome").exists();
    browsers.push(("Chrome".to_string(), chrome_exists));

    let canary_exists = get_browser_base_dir("Chrome Canary").exists();
    browsers.push(("Chrome Canary".to_string(), canary_exists));

    browsers
}
