// pub mod fabric::secrets;
use crate::fabric::secrets::get_secret;
use crate::fabric::secrets::update_secret;
use crate::state::AppState;
use std::process::Command;
use tauri::State;

#[tauri::command]
pub fn set_model(state: State<AppState>, value: String) {
    let mut model = state.model.lock().unwrap();
    *model = value;
}

#[tauri::command]
pub fn get_model(state: State<AppState>) -> String {
    let model = state.model.lock().unwrap();
    model.clone()
}
#[tauri::command]
pub fn get_models(_app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let output = Command::new("fabric")
        .arg("--listmodels")
        .output()
        .map_err(|e| format!("Failed to execute fabric command: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let models: Vec<String> = output_str
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(models)
}

#[tauri::command]
pub async fn set_default_model(app: tauri::AppHandle, model: String) -> Result<(), String> {
    update_secret(app, "DEFAULT_MODEL".to_string(), model).await
}

#[tauri::command]
pub async fn get_default_model(app: tauri::AppHandle) -> Result<String, String> {
    get_secret(app, "DEFAULT_MODEL".to_string()).await
}
