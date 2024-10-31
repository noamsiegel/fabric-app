use crate::state::AppState;
use tauri::{AppHandle, Error, State};
use tauri_plugin_shell::ShellExt;

// TODO get the jina ai functions to work from the rust side
// TODO change function to "run_fabric_pattern"
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

    // Run command using shell plugin
    let shell_command = format!(
        " {} {} | fabric --pattern \"{}\"",
        flag, input, selected_pattern
    );
    println!("Executing shell command: {}", shell_command);

    let output = app
        .shell()
        .command("fabric")
        .args(&[&shell_command])
        .output()
        .await
        .map_err(|_| Error::FailedToReceiveMessage)?;

    println!("Output: {}", String::from_utf8_lossy(&output.stdout));

    let result = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Command output: {}", result);

    // Reset running state
    *state
        .is_running
        .lock()
        .map_err(|_| Error::FailedToReceiveMessage)? = false;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
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
    let clipboard_command = match tauri_plugin_os::platform() {
        "windows" => "powershell.exe -command \"Get-Clipboard\"",
        "macos" => "pbpaste",
        "linux" => "xclip -selection clipboard -o",
        _platform => {
            *state
                .is_running
                .lock()
                .map_err(|_| Error::FailedToReceiveMessage)? = false;
            return Err(Error::FailedToReceiveMessage);
        }
    };

    println!("Clipboard command: {}", clipboard_command);

    // Run command using shell plugin
    let shell_command = format!(
        "{} | fabric --pattern \"{}\"",
        clipboard_command, selected_pattern
    );
    println!("Executing shell command: {}", shell_command);

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
