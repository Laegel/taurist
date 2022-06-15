use tauri::{Manager, WindowBuilder};

const SPLASHSCREEN_WINDOW_LABEL: &str = "splashscreen";

#[tauri::command]
pub async fn open_splashscreen(app_handle: tauri::AppHandle) -> Result<(), String> {
    WindowBuilder::new(
        &app_handle,
        SPLASHSCREEN_WINDOW_LABEL,
        tauri::WindowUrl::App("/splashscreen.html".into()),
    )
    .title("Tauri - Splashscreen")
    .center()
    .inner_size(750.0, 300.0)
    .resizable(false)
    .decorations(false)
    .build()
    .unwrap();
    Ok(())
}

#[tauri::command]
pub async fn close_splashscreen(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle.get_window(SPLASHSCREEN_WINDOW_LABEL).unwrap().close();
    Ok(())
}
