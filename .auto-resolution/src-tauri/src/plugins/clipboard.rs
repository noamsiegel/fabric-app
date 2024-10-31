use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
pub fn get_clipboard_contents(app_handle: tauri::AppHandle) -> Result<String, String> {
    app_handle
        .clipboard()
        .read_text()
        .map_err(|e| e.to_string())
}
