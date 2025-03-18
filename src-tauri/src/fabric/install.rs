use std::process::Command;
use tauri::command;
use std::path::PathBuf;

#[command]
pub async fn install_fabric() -> Result<String, String> {
    // Check if Go is installed
    let go_check = Command::new("go").arg("version").output();
    if let Err(e) = go_check {
        return Err(format!("Go is not installed or not in PATH: {}", e));
    }
    
    // Install fabric
    let output = Command::new("go")
        .args(["install", "github.com/danielmiessler/fabric@latest"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Fabric installed successfully".to_string())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to install Fabric: {}", error_message))
    }
}

#[command]
pub async fn check_fabric_installation() -> Result<bool, String> {
    // Try multiple ways to find fabric
    
    // 1. Check D:\fabric\fabric.exe specifically for the user's case
    #[cfg(target_os = "windows")]
    {
        let d_fabric_path = PathBuf::from(r"D:\fabric\fabric.exe");
        if d_fabric_path.exists() {
            println!("Found fabric at D:\\fabric\\fabric.exe");
            return Ok(true);
        }
    }
    
    // 2. Try the normal command lookup
    #[cfg(target_os = "windows")]
    let fabric_cmd = "fabric.exe";
    
    #[cfg(not(target_os = "windows"))]
    let fabric_cmd = "fabric";
    
    let result = Command::new(fabric_cmd).arg("--version").output();
    match result {
        Ok(output) if output.status.success() => {
            println!("Fabric found in PATH");
            return Ok(true);
        }
        _ => {
            // 3. Final check: Try using where/which command
            #[cfg(target_os = "windows")]
            let where_result = Command::new("where").arg("fabric").output();
            
            #[cfg(not(target_os = "windows"))]
            let where_result = Command::new("which").arg("fabric").output();
            
            match where_result {
                Ok(output) if output.status.success() => {
                    println!("Fabric found via where/which command");
                    return Ok(true);
                }
                _ => {
                    println!("Fabric not found");
                    return Ok(false);
                }
            }
        }
    }
}
