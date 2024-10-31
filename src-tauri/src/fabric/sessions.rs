use tauri::{Error, Manager};
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub async fn set_session(app: tauri::AppHandle, session: String) -> Result<String, Error> {
    let shell = app.shell();
    let output = shell
        .command("fabric")
        .args(["--session", &session])
        .output()
        .await
        .map_err(|_| Error::FailedToReceiveMessage)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(Error::FailedToReceiveMessage)
    }
}

#[tauri::command]
pub async fn list_sessions(app: tauri::AppHandle) -> Result<String, Error> {
    let shell = app.shell();
    let output = shell
        .command("fabric")
        .args(["-X"])
        .output()
        .await
        .map_err(|_| Error::FailedToReceiveMessage)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(Error::FailedToReceiveMessage)
    }
}

#[tauri::command]
pub async fn output_session(app: tauri::AppHandle) -> Result<String, Error> {
    let shell = app.shell();
    let output = shell
        .command("fabric")
        .args(["--output-session"])
        .output()
        .await
        .map_err(|_| Error::FailedToReceiveMessage)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(Error::FailedToReceiveMessage)
    }
}

#[tauri::command]
pub async fn wipe_session(app: tauri::AppHandle, session: String) -> Result<String, Error> {
    let shell = app.shell();
    let output = shell
        .command("fabric")
        .args(["-W", &session])
        .output()
        .await
        .map_err(|_| Error::FailedToReceiveMessage)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(Error::FailedToReceiveMessage)
    }
}

#[tauri::command]
pub async fn print_session(app: tauri::AppHandle, session: String) -> Result<String, Error> {
    let shell = app.shell();
    let output = shell
        .command("fabric")
        .args(["--printsession", &session])
        .output()
        .await
        .map_err(|_| Error::FailedToReceiveMessage)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(Error::FailedToReceiveMessage)
    }
}
