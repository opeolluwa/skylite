// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ipc;
use server;

use tauri::Manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() {
    // run core the server in a separate thread from tauri
    tauri::async_runtime::spawn(server::run());

    tauri::Builder::default()
        .setup(|app| {
            // emit the `event-name` event to all webview windows on the frontend
            app.emit_all(
                "device-connected",
                Payload {
                    message: "contact made on the frontend".into(),
                },
            )
            .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::server_address::server,
            ipc::wifi::is_connected_to_wifi
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
