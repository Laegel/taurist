use tauri::{self, TrayIcon};

#[tauri::command]
pub fn remove_system_tray(app: tauri::AppHandle) {
    app.tray_handle().set_icon(TrayIcon::File("../icons/icon-warning.png".into()));
}
