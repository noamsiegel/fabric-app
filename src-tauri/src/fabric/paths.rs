use std::path::PathBuf;
use tauri::Manager;
use std::env;
use std::process::Command;

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
/// - **Windows:** C:\Users\{user}\.config\fabric
#[tauri::command]
pub async fn get_fabric_config_dir(app: tauri::AppHandle) -> Result<PathBuf, String> {
    // Check for custom config path in environment
    if let Ok(custom_path) = env::var("FABRIC_CONFIG_DIR") {
        let path = PathBuf::from(custom_path);
        if path.exists() {
            return Ok(path);
        }
    }

    // Get the config directory using the path resolver
    let mut config_dir = app
        .path()
        .home_dir()
        .map_err(|_| "Could not find home directory".to_string())?;

    // Append '.config/fabric' to the config path
    config_dir.push(".config");
    config_dir.push("fabric");

    println!("Fabric config directory: {:?}", config_dir);
    
    // Create directory if it doesn't exist
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Could not create config directory: {}", e))?;
    }

    Ok(config_dir)
}

/// Returns the path to the fabric bin file
///
/// ## Platform-specific
///
/// - **MacOS:** /Users/{user}/go/bin/fabric
/// - **Windows:** Various locations checked in order:
///   1. Custom path from FABRIC_BIN_PATH env var
///   2. D:\fabric\fabric.exe
///   3. PATH environment
///   4. %USERPROFILE%\go\bin\fabric.exe
#[tauri::command]
pub async fn get_fabric_bin_path(app: tauri::AppHandle) -> Result<PathBuf, String> {
    // First check for custom path in environment
    if let Ok(custom_path) = env::var("FABRIC_BIN_PATH") {
        let path = PathBuf::from(custom_path);
        if path.exists() {
            println!("Using custom fabric path from env: {:?}", path);
            return Ok(path);
        }
    }
    
    // For Windows, check D:\fabric directly first
    #[cfg(target_os = "windows")]
    {
        let d_fabric_path = PathBuf::from(r"D:\fabric\fabric.exe");
        if d_fabric_path.exists() {
            println!("Found fabric at D:\\fabric\\fabric.exe");
            return Ok(d_fabric_path);
        }
        
        // Also try D:\fabric\bin\fabric.exe
        let d_fabric_bin_path = PathBuf::from(r"D:\fabric\bin\fabric.exe");
        if d_fabric_bin_path.exists() {
            println!("Found fabric at D:\\fabric\\bin\\fabric.exe");
            return Ok(d_fabric_bin_path);
        }
    }

    // Then check if fabric is available in PATH
    #[cfg(target_os = "windows")]
    let where_cmd = Command::new("where").arg("fabric").output();
    
    #[cfg(not(target_os = "windows"))]
    let where_cmd = Command::new("which").arg("fabric").output();
    
    if let Ok(output) = where_cmd {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout);
            let first_line = path_str.lines().next().unwrap_or("");
            if !first_line.is_empty() {
                let path = PathBuf::from(first_line.trim());
                println!("Found fabric in PATH: {:?}", path);
                return Ok(path);
            }
        }
    }

    // Fallback to the default location
    let mut path = app
        .path()
        .home_dir()
        .map_err(|_| "Could not find home directory".to_string())?;

    // Build the path based on platform
    #[cfg(target_os = "windows")]
    {
        path.push("go");
        path.push("bin");
        path.push("fabric.exe");
    }

    #[cfg(not(target_os = "windows"))]
    {
        path.push("go");
        path.push("bin");
        path.push("fabric");
    }

    println!("Checking default fabric path: {:?}", path);

    // Provide a warning if the file doesn't exist
    if !path.exists() {
        println!("Warning: Fabric not found at {}", path.display());
        // Fallback to just "fabric" and hope it's in PATH during execution
        #[cfg(target_os = "windows")]
        return Ok(PathBuf::from("fabric.exe"));
        
        #[cfg(not(target_os = "windows"))]
        return Ok(PathBuf::from("fabric"));
    }

    Ok(path)
}

/// Converts a PathBuf to a String
pub fn path_to_string(path: PathBuf) -> Result<String, String> {
    path.to_str()
        .ok_or_else(|| "Failed to convert path to string".to_string())
        .map(|s| s.to_string())
}
