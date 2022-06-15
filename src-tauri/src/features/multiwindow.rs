use tauri::{AppHandle, Manager, WindowBuilder};

fn open(app_handle: AppHandle, label: &str) {
    WindowBuilder::new(
        &app_handle,
        label,
        tauri::WindowUrl::App("/multiwindow.html".into()),
    )
    .title("Tauri - Multiwindow")
    .center()
    .inner_size(100.0, 100.0)
    .resizable(false)
    .decorations(false)
    .build()
    .unwrap();
}

#[tauri::command]
pub async fn open_multiwindow(app_handle: AppHandle) -> Result<(), String> {
    open(app_handle, "Multiwindow1");
    Ok(())
}

#[tauri::command]
pub async fn close_multiwindow(app_handle: AppHandle) -> Result<(), String> {
    app_handle.get_window("Multiwindow1").unwrap().close();
    Ok(())
}

// #[tauri::command]
// pub async fn open_all_multiwindows(app_handle: AppHandle) -> Result<(), String> {
//     for i in [1..3].iter() {
//         open(&mut app_handle, &format!("Multiwindow{}", i));
//     }
//     Ok(())
// }

// #[tauri::command]
// pub async fn close_all_multiwindows(app_handle: AppHandle) -> Result<(), String> {
//     app_handle.windows().iter().filter(|(key, _)| key.starts_with("Multiwindow")).map(|(_, window)| window.close());
//     Ok(())
// }