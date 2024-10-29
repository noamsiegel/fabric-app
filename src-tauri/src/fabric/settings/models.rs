// pub mod fabric::secrets;
use crate::fabric::secrets::get_secret;
use crate::fabric::secrets::update_secret;
use crate::state::AppState;
use std::process::Command;
use tauri::State;
#[tauri::command]
pub fn set_temperature(state: State<AppState>, value: f32) {
    let mut temperature = state.temperature.lock().unwrap();
    *temperature = value;
}

#[tauri::command]
pub fn get_temperature(state: State<AppState>) -> f32 {
    let temperature = state.temperature.lock().unwrap();
    *temperature
}

#[tauri::command]
pub fn set_presence_penalty(state: State<AppState>, value: f32) {
    let mut presence_penalty = state.presence_penalty.lock().unwrap();
    *presence_penalty = value;
}

#[tauri::command]
pub fn get_presence_penalty(state: State<AppState>) -> f32 {
    let presence_penalty = state.presence_penalty.lock().unwrap();
    *presence_penalty
}

#[tauri::command]
pub fn set_top_p(state: State<AppState>, value: f32) {
    let mut top_p = state.top_p.lock().unwrap();
    *top_p = value;
}

#[tauri::command]
pub fn get_top_p(state: State<AppState>) -> f32 {
    let top_p = state.top_p.lock().unwrap();
    *top_p
}

#[tauri::command]
pub fn set_frequency_penalty(state: State<AppState>, value: f32) {
    let mut frequency_penalty = state.frequency_penalty.lock().unwrap();
    *frequency_penalty = value;
}

#[tauri::command]
pub fn get_frequency_penalty(state: State<AppState>) -> f32 {
    let frequency_penalty = state.frequency_penalty.lock().unwrap();
    *frequency_penalty
}

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

    println!("vendors: {:?}", vendors);

    Ok(vendors)
}

#[tauri::command]
pub async fn set_default_model(app: tauri::AppHandle, model: String) -> Result<(), String> {
    update_secret(app, "DEFAULT_MODEL".to_string(), model).await
}

#[tauri::command]
pub async fn set_default_vendor(app: tauri::AppHandle, vendor: String) -> Result<(), String> {
    update_secret(app, "DEFAULT_VENDOR".to_string(), vendor).await
}

#[tauri::command]
pub async fn get_default_model(app: tauri::AppHandle) -> Result<String, String> {
    get_secret(app, "DEFAULT_MODEL".to_string()).await
}

#[tauri::command]
pub async fn get_default_vendor(app: tauri::AppHandle) -> Result<String, String> {
    get_secret(app, "DEFAULT_VENDOR".to_string()).await
}
