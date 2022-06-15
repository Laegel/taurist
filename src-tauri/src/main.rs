#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent, CustomMenuItem};
use tauri::Manager;

mod experience;
mod features;
mod wizard;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new();
//   .add_item(quit)
//   .add_native_item(SystemTrayMenuItem::Separator)
//   .add_item(hide);

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            

            if !wizard::is_installed() {
                wizard::install();
            }
            // wizard::trigger();

            if let Err(config) = experience::get_config() {
                app.emit_all("configure", {});
            }

            Ok(())
        })
        .manage(features::commands::Database {})
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                // get a handle to the clicked menu item
                // note that `tray_handle` can be called anywhere,
                // just get a `AppHandle` instance with `app.handle()` on the setup hook
                // and move it to another function or thread
                
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                        // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
                        item_handle.set_title("Show").unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            experience::get_config,
            experience::set_config,
            // features::events::run_in_setup(app);
            features::open_preview_window,
            features::close_preview_window,
            features::commands::my_custom_command,
            features::splashscreen::open_splashscreen,
            features::splashscreen::close_splashscreen,
            features::multiwindow::open_multiwindow,
            features::system_tray::remove_system_tray,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
