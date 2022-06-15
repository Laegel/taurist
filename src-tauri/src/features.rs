use tauri::{Manager, Menu, PhysicalPosition, Position, WindowBuilder, WindowEvent};

pub mod commands;
pub mod events;
pub mod menu;
pub mod multiwindow;
pub mod splashscreen;
pub mod system_tray;

const PREVIEW_WINDOW_LABEL: &str = "preview";

#[tauri::command]
pub async fn open_preview_window(
    feature: String,
    window: tauri::Window,
    app_handle: tauri::AppHandle,
) {
    let menu = if feature == "menu".to_string() {
        menu::get_custom_menu()
    } else {
        Menu::new()
    };

    match app_handle.get_window(PREVIEW_WINDOW_LABEL) {
        Some(window) => {
            window.close();
            ()
        }
        None => (),
    }
    const preview_window_length: f64 = 400.0;
    let window_size = window.outer_size().unwrap();
    let window_position = window.outer_position().unwrap();

    let preview_window = WindowBuilder::new(
        &app_handle,
        PREVIEW_WINDOW_LABEL.to_string(),
        tauri::WindowUrl::App(format!("/#preview-{}", feature).into()),
    )
    .title("Tauri - Preview")
    .position(
        window_position.x as f64 + window_size.width as f64,
        window_position.y as f64,
    )
    .inner_size(preview_window_length, window_size.height as f64)
    .resizable(false)
    .menu(menu)
    .decorations(false)
    .build()
    .unwrap();

    window.on_window_event(move |event| match event {
        WindowEvent::Moved(position) => {
            let preview_window_position = PhysicalPosition {
                x: position.x + window_size.width as i32,
                y: position.y + 0,
            };
            preview_window.set_position(Position::Physical(preview_window_position));
        }
        WindowEvent::Focused(is_focused) => {}
        _ => (),
    });
}

#[tauri::command]
pub async fn close_preview_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle.get_window(PREVIEW_WINDOW_LABEL).unwrap().close();
    Ok(())
}
