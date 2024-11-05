use std::path::PathBuf;
use tauri::Manager;

/// Returns the path to the user's home directory
///
/// ## Platform-specific
///
/// - **MacOS:** /Users/{user}
/// - **Windows:** C:\Users\{user}
#[tauri::command]
pub async fn get_home_dir(app: tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .home_dir()
        .map_err(|_| "Could not find home directory".to_string())
}

/// Returns the path to the fabric config directory
///
/// ## Platform-specific
///
/// - **MacOS:** /Users/{user}/.config/fabric
/// - **Windows:**
#[tauri::command]
pub async fn get_fabric_config_dir(app: tauri::AppHandle) -> Result<PathBuf, String> {
    // Get the config directory using the path resolver
    let mut config_dir = app
        .path()
        .home_dir()
        .map_err(|_| "Could not find config directory".to_string())?;

    // Append 'fabric' to the config path
    config_dir.push(".config");
    config_dir.push("fabric");

    Ok(config_dir)
}

/// Returns the path to the fabric bin file
///
/// ## Platform-specific
///
/// - **MacOS:** /Users/{user}/go/bin/fabric
/// - **Windows:**
#[tauri::command]
pub async fn get_fabric_bin_path(app: tauri::AppHandle) -> Result<PathBuf, String> {
    // Get home directory using the path resolver
    let mut path = app
        .path()
        .home_dir()
        .map_err(|_| "Could not find home directory".to_string())?;

    // Build the path: /Users/{user}/go/bin/fabric
    path.push("go");
    path.push("bin");
    path.push("fabric");

    println!("Fabric bin path: {:?}", path);

    Ok(path)
}
/// Converts a PathBuf to a String
pub fn path_to_string(path: PathBuf) -> Result<String, String> {
    path.to_str()
        .ok_or_else(|| "Failed to convert path to string".to_string())
        .map(|s| s.to_string())
}
