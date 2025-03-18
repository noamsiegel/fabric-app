use crate::state::AppState;
use crate::fabric::paths::get_fabric_bin_path;
use std::process::Command;
use tauri::{AppHandle, Error, State};
use tauri_plugin_shell::ShellExt;

// TODO get the jina ai functions to work from the rust side
// TODO change function to "run_fabric_pattern"
// Issue URL: https://github.com/noamsiegel/fabric-app/issues/82
#[tauri::command]
pub async fn run_fabric_command(
    app: AppHandle,
    input: String,
    flag: String,
    state: State<'_, AppState>,
) -> Result<String, Error> {
    // Set running state
    *state
        .is_running
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)? = true;

    // Get selected pattern
    let selected_pattern = state
        .selected_pattern
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)?
        .clone();

    println!("Selected pattern: {}", selected_pattern);

    // Check if pattern is None
    if selected_pattern.is_empty() {
        // Reset running state before returning error
        *state
            .is_running
            .lock()
            .map_err(|_| Error::FailedToReceiveMessage)? = false;

        return Err(Error::FailedToReceiveMessage);
    }

    // Try to get the direct fabric path
    let fabric_path_result = get_fabric_bin_path(app.clone()).await;
    
    let result = if let Ok(fabric_path) = fabric_path_result {
        // If we have a direct path, use it
        println!("Using fabric path: {:?}", fabric_path);
        
        // On Windows, we need to handle command execution differently
        #[cfg(target_os = "windows")]
        let output = {
            // First try running directly (works with .exe extension)
            let direct_result = Command::new(&fabric_path)
                .args([flag.as_str(), &input, "--pattern", &selected_pattern])
                .output();
                
            if let Ok(output) = direct_result {
                Ok(output)
            } else {
                // Fallback to cmd.exe if direct execution fails
                let fabric_command = format!(
                    "{} {} {} --pattern \"{}\"",
                    fabric_path.to_string_lossy(),
                    flag,
                    input,
                    selected_pattern
                );
                
                Command::new("cmd")
                    .args(["/C", &fabric_command])
                    .output()
            }
        };

        #[cfg(not(target_os = "windows"))]
        let output = Command::new(&fabric_path)
            .args([flag.as_str(), &input, "--pattern", &selected_pattern])
            .output();
            
        match output {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).to_string()
            },
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("Error: {}", stderr)
            },
            Err(e) => {
                println!("Command execution error: {}", e);
                format!("Failed to execute fabric: {}", e)
            },
        }
    } else {
        // Fallback to shell plugin if direct path fails
        println!("Could not get fabric path, falling back to shell plugin");
        
        #[cfg(target_os = "windows")]
        let shell_command = format!(
            "fabric.exe {} {} --pattern \"{}\"", 
            flag, input, selected_pattern
        );
        
        #[cfg(not(target_os = "windows"))]
        let shell_command = format!(
            "fabric {} {} --pattern \"{}\"", 
            flag, input, selected_pattern
        );
        
        println!("Falling back to shell plugin with command: {}", shell_command);
        
        match app.shell().command(shell_command).output() {
            Ok(output) => {
                if output.status.success() {
                    output.stdout
                } else {
                    format!("Error: {}", output.stderr)
                }
            },
            Err(e) => format!("Shell execution failed: {}", e),
        }
    };

    // Reset running state
    *state
        .is_running
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)? = false;

    Ok(result)
}

#[tauri::command]
pub async fn scrape_url_and_run_pattern(
    app: AppHandle,
    url: String,
    state: State<'_, AppState>,
) -> Result<String, Error> {
    run_fabric_command(app, url, "-u".into(), state).await
}

#[tauri::command]
pub async fn scrape_question_and_run_pattern(
    app: AppHandle,
    question: String,
    state: State<'_, AppState>,
) -> Result<String, Error> {
    run_fabric_command(app, question, "-q".into(), state).await
}

#[tauri::command]
pub async fn clipboard_contents_and_run_pattern(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, Error> {
    // Set running state
    *state
        .is_running
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)? = true;

    // Get selected pattern
    let selected_pattern = state
        .selected_pattern
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)?
        .clone();

    // Check if pattern is None
    if selected_pattern.is_empty() {
        // Reset running state before returning error
        *state
            .is_running
            .lock()
            .map_err(|_| Error::FailedToReceiveMessage)? = false;
        return Ok("Please select a pattern first.".to_string());
    }

    // Get platform-specific clipboard command
    #[allow(unused_assignments)]
    let mut clipboard_command = String::new();
    
    #[cfg(target_os = "windows")]
    {
        // Fix PowerShell command to handle multiline content better
        clipboard_command = "powershell.exe -command \"[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-Clipboard\"".to_string();
    }
    
    #[cfg(target_os = "macos")]
    {
        clipboard_command = "pbpaste".to_string();
    }
    
    #[cfg(target_os = "linux")]
    {
        clipboard_command = "xclip -selection clipboard -o".to_string();
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        *state
            .is_running
            .lock()
            .map_err(|_| Error::FailedToReceiveMessage)? = false;
        return Err(Error::FailedToReceiveMessage);
    }

    println!("Clipboard command: {}", clipboard_command);

    // Get fabric path
    let fabric_path_result = get_fabric_bin_path(app.clone()).await;
    let fabric_bin = if let Ok(path) = fabric_path_result {
        path.to_string_lossy().to_string()
    } else {
        #[cfg(target_os = "windows")]
        let default = "fabric.exe".to_string();
        
        #[cfg(not(target_os = "windows"))]
        let default = "fabric".to_string();
        
        default
    };

    // Construct and run command
    #[cfg(target_os = "windows")]
    let shell_command = format!(
        "{} | {} --pattern \"{}\"",
        clipboard_command, fabric_bin, selected_pattern
    );
    
    #[cfg(not(target_os = "windows"))]
    let shell_command = format!(
        "{} | {} --pattern \"{}\"",
        clipboard_command, fabric_bin, selected_pattern
    );
    
    println!("Executing shell command: {}", shell_command);

    #[cfg(target_os = "windows")]
    let output = app
        .shell()
        .command("cmd")
        .args(&["/C", &shell_command])
        .output()
        .await
        .map_err(|e| {
            println!("Clipboard command error: {:?}", e);
            Error::FailedToReceiveMessage
        })?;
        
    #[cfg(not(target_os = "windows"))]
    let output = app
        .shell()
        .command("sh")
        .args(&["-c", &shell_command])
        .output()
        .await
        .map_err(|_| Error::FailedToReceiveMessage)?;

    // Reset running state
    *state
        .is_running
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)? = false;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

// Get and set running state
#[tauri::command]
pub fn get_is_running(state: tauri::State<AppState>) -> bool {
    let is_running = state.is_running.lock().unwrap();
    *is_running
}

#[tauri::command]
pub fn set_is_running(state: tauri::State<AppState>, value: bool) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = value;
}
