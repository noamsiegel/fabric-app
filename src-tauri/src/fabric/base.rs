use std::fs;
use tauri::Manager;

#[tauri::command]
pub async fn get_home_dir(app: tauri::AppHandle) -> Result<String, String> {
    // Get the home directory using the path resolver
    let home = app
        .path()
        .home_dir()
        .map_err(|_| "Could not find home directory".to_string())?;

    // Convert the PathBuf to a String
    let home_str = home
        .to_str()
        .ok_or_else(|| "Could not convert home path to string".to_string())?
        .to_string();

    Ok(home_str)
}

#[tauri::command]
pub async fn get_fabric_dir(app: tauri::AppHandle) -> Result<String, String> {
    // Get the config directory using the path resolver
    let config_dir = app
        .path()
        .resolve("fabric/patterns", tauri::path::BaseDirectory::Config)
        .map_err(|_| "Could not resolve patterns directory".to_string())?;

    // Convert the PathBuf to a String
    let patterns_dir = config_dir
        .to_str()
        .ok_or_else(|| "Could not convert patterns path to string".to_string())?
        .to_string();

    Ok(patterns_dir)
}

#[tauri::command]
pub async fn get_pattern_folders(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    // Get the home directory and construct the path to .config/fabric/patterns
    let mut patterns_dir = app
        .path()
        .home_dir()
        .map_err(|_| "Could not find home directory".to_string())?;

    // Append .config/fabric/patterns to the home directory
    patterns_dir.push(".config");
    patterns_dir.push("fabric");
    patterns_dir.push("patterns");

    // Create directories if they don't exist
    fs::create_dir_all(&patterns_dir)
        .map_err(|_| "Could not create patterns directory".to_string())?;

    println!("patterns_dir: {:?}", patterns_dir);

    // Read the directory entries
    let entries =
        fs::read_dir(&patterns_dir).map_err(|_| "Could not read patterns directory".to_string())?;

    // Filter for directories and collect their names
    let folders: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            // Only include if it's a directory
            if entry.file_type().ok()?.is_dir() {
                // Convert the filename to a string
                entry.file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect();

    Ok(folders)
}
