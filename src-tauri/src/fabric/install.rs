use std::process::Command;
use tauri::command;

#[command]
pub async fn install_fabric() -> Result<String, String> {
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
