use serde;
use tauri::{App, Manager};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    message: String,
}

pub fn run_in_setup(app: &mut App) {

    let main_window = app.get_window("main").unwrap();
    let main_window2 = app.get_window("main").unwrap();
    
    main_window.listen_global("ping", move |event| {
        println!("got ping event with payload {:?}", event.payload());


        main_window2.emit_all(
            "pong",
            Payload {
                message: "Tauri is awesome!".into(),
            },
        )
        .unwrap();
    });
    // unlisten to the event using the `id` returned on the `listen_global` function
    // an `once_global` API is also exposed on the `App` struct

    // emit the `event-name` event to all webview windows on the frontend
}
