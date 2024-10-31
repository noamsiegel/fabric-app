// pub mod fabric::secrets;
use crate::fabric::secrets::{get_secret, update_secret};
use std::process::Command;

#[tauri::command]
pub async fn get_default_vendor(app: tauri::AppHandle) -> Result<String, String> {
    get_secret(app, "DEFAULT_VENDOR".to_string()).await
}

#[tauri::command]
pub async fn set_default_vendor(app: tauri::AppHandle, vendor: String) -> Result<(), String> {
    update_secret(app, "DEFAULT_VENDOR".to_string(), vendor).await
}

#[tauri::command]
pub fn get_vendors(_app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let output = Command::new("fabric")
        .arg("--listmodels")
        .output()
        .map_err(|e| format!("Failed to execute fabric command: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let vendors: Vec<String> = output_str
        .lines()
        .filter(|line| {
            !line.trim().starts_with('[')
                && !line.trim().starts_with("Available models:")
                && !line.trim().is_empty()
        })
        .map(|line| line.trim().to_string())
        .collect();

    Ok(vendors)
}
