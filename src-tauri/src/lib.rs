mod services;

use services::{OcrService, WsClient};
use std::sync::Arc;
use tauri::Manager;
use tokio::time::{sleep, Duration};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                print!("Starting OCR process...");
                let ocr_process = Arc::new(OcrService::new());

                if let Err(e) = ocr_process.start() {
                    eprintln!("Failed to start OCR: {}", e);
                    return;
                }

                println!("Waiting for Websocket server...");
                if !wait_for_websocket("ws://localhost:8765", 10).await {
                    eprintln!("Websocket server didn't start in time");
                    return;
                }
                println!("Websocket sever is ready");

                let ws_client = Arc::new(WsClient::new("ws://localhost:8765"));

                if let Err(e) = register_screenshot_hotkey(&app_handle, ws) {
                    eprintln!("Hotkey failed: {}", e);
                    return;
                }

                println!("Ready! Press Cmd+D to capture");
                // setup health_check here

                app_handle.manage(ocr_process);
                app_handle.manage(ws_client);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn wait_for_websocket(url: &str, timeout_secs: u64) -> bool {
    use tokio_tungstenite::connect_async;

    for i in 0..timeout_secs {
        sleep(Duration::from_secs(1)).await;

        if let Ok(_) = connect_async(url).await {
            return true;
        }

        if i % 2 == 0 {
            println!("Still waiting... ({}/{}s)", i + 1, timeout_secs)
        }
    }

    false
}
