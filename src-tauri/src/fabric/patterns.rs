use crate::fabric::secrets::{get_secret, update_secret};
use crate::state::AppState;
use std::fs;
use std::process::Command;
use tauri::Manager;
use tauri::{Error, State};
use tauri_plugin_shell::ShellExt;

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
pub async fn get_patterns(app: tauri::AppHandle) -> Result<Vec<String>, String> {
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

#[tauri::command]
pub async fn set_selected_pattern(
    pattern: String,
    state: State<'_, AppState>,
) -> Result<(), Error> {
    // Lock the mutex to get mutable access
    let mut selected_pattern = state
        .selected_pattern
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)?;

    // Set the new pattern
    *selected_pattern = pattern;

    Ok(())
}

#[tauri::command]
pub async fn get_selected_pattern(state: State<'_, AppState>) -> Result<String, Error> {
    // Lock the mutex to get access to the pattern
    let selected_pattern = state
        .selected_pattern
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)?;

    // Clone the pattern to return it
    Ok(selected_pattern.clone())
}

#[tauri::command]
pub async fn set_patterns_git_repo(app: tauri::AppHandle, repo_url: String) -> Result<(), String> {
    update_secret(app, "PATTERNS_LOADER_GIT_REPO_URL".to_string(), repo_url).await
}

#[tauri::command]
pub async fn set_patterns_git_folder(
    app: tauri::AppHandle,
    folder_path: String,
) -> Result<(), String> {
    update_secret(
        app,
        "PATTERNS_LOADER_GIT_REPO_PATTERNS_FOLDER".to_string(),
        folder_path,
    )
    .await
}

#[tauri::command]
pub async fn get_patterns_git_repo(app: tauri::AppHandle) -> Result<String, String> {
    get_secret(app, "PATTERNS_LOADER_GIT_REPO_URL".to_string()).await
}

#[tauri::command]
pub async fn get_patterns_git_folder(app: tauri::AppHandle) -> Result<String, String> {
    get_secret(app, "PATTERNS_LOADER_GIT_REPO_PATTERNS_FOLDER".to_string()).await
}

// TODO make sure that this works, it doesn't seem to be updating
#[tauri::command]
pub async fn update_patterns(app: tauri::AppHandle) -> Result<String, Error> {
    println!("Starting pattern update...");

    let shell = app.shell();
    let output = shell
        .command("/usr/local/bin/fabric")
        .args(["-U"])
        .output()
        .await
        .map_err(|e| {
            println!("Error executing fabric command: {:?}", e);
            Error::FailedToReceiveMessage
        })?;

    println!("Command completed with status: {:?}", output.status);

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).into_owned();
        println!("Update successful: {}", result);
        Ok(result)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Update failed: {}", error);
        Err(Error::FailedToReceiveMessage)
    }
}

#[tauri::command]
pub fn run_fabric(_app_handle: tauri::AppHandle, flag: String) -> Result<String, Error> {
    let output = Command::new("/usr/local/bin/fabric")
        .arg(&flag)
        .output()
        .map_err(|e| {
            // Log the error if the command fails to start
            println!("Failed to start fabric command for flag '{}'. Error: {}", flag, e);
            Error::FailedToReceiveMessage
        })?;

    if !output.status.success() {
        // Log the stderr to the console when the command fails
        let stderr_output = String::from_utf8_lossy(&output.stderr);
        let stdout_output = String::from_utf8_lossy(&output.stdout);
        println!("Fabric command for flag '{}' failed. Status: {:?}. Stderr: {}. Stdout: {}", flag, output.status, stderr_output, stdout_output);
        return Err(Error::FailedToReceiveMessage);
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[tauri::command]
pub async fn set_default_pattern(pattern: String, state: State<'_, AppState>) -> Result<(), Error> {
    // Lock the mutex to get mutable access
    let mut default_pattern = state
        .default_pattern
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)?;

    // Set the new default pattern
    *default_pattern = pattern;

    Ok(())
}

#[tauri::command]
pub async fn get_default_pattern(state: State<'_, AppState>) -> Result<String, Error> {
    // Lock the mutex to get access to the pattern
    let default_pattern = state
        .default_pattern
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)?;

    // Clone the pattern to return it
    Ok(default_pattern.clone())
}
