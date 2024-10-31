use crate::fabric::patterns::run_fabric;
use tauri::{AppHandle, Error};

#[tauri::command]
pub async fn set_session(app: AppHandle, session: String) -> Result<String, Error> {
    run_fabric(app, format!("--session={}", session))
}

#[tauri::command]
pub async fn list_sessions(app: AppHandle) -> Result<String, Error> {
    run_fabric(app, "--listsessions".to_string())
}

#[tauri::command]
pub async fn output_session(app: AppHandle) -> Result<String, Error> {
    run_fabric(app, "--output-session".to_string())
}

#[tauri::command]
pub async fn wipe_session(app: AppHandle, session: String) -> Result<String, Error> {
    run_fabric(app, format!("--wipesession={}", session))
}

#[tauri::command]
pub async fn print_session(app: AppHandle, session: String) -> Result<String, Error> {
    run_fabric(app, format!("--printsession={}", session))
}
