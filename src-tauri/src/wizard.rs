use std::{fs, process::Command};

use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};

fn get_storage_file_path() -> String {
    format!(
        "{}/{}",
        tauri::api::path::data_dir().unwrap().display(),
        "tauri-init.sh"
    )
}

pub fn install() {
    let client = ClientBuilder::new().build().unwrap();
    let request = HttpRequestBuilder::new(
        "GET",
        "https://raw.githubusercontent.com/chippers/tauri-init/main/docs/tauri-init.sh",
    )
    .unwrap()
    .response_type(ResponseType::Text);

    tauri::async_runtime::spawn(async move {
        if let Ok(response) = client.send(request).await {
            let mut content = String::from("# v0.0.13\n");
            content.push_str(response.read().await.unwrap().data.as_str().unwrap());
            fs::write(get_storage_file_path(), content).expect("Failed to save tauri-init.sh");
        } else {
            println!("Something Happened!");
        }
    });
}

pub fn is_installed() -> bool {
    match fs::read(get_storage_file_path()) {
        Ok(_) => true,
        Err(_e) => false,
    }
}

pub fn trigger() {
    let output = Command::new("sh")
        .arg(get_storage_file_path())
        .arg("v0.0.13")
        .output()
        .expect("Failed to execute command");
    println!("{:?}", output);
}
