mod services;

use once_cell::sync::OnceCell;
use services::{register_screenshot_hotkey, OcrService, WsClient};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

static WS_CLIENT: OnceCell<Arc<WsClient>> = OnceCell::new();
static OCR_PROCESS: OnceCell<Arc<OcrService>> = OnceCell::new();

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let ocr_process = Arc::new(OcrService::new());

                if let Err(e) = ocr_process.start() {
                    eprintln!("Failed to start OCR: {}", e);
                    return;
                }

                if !wait_for_websocket_port(8765, 10).await {
                    eprintln!("WebSocket server didn't start");
                    return;
                }

                let ws_client = Arc::new(WsClient::new("ws://localhost:8765"));

                OCR_PROCESS.set(ocr_process).ok();
                WS_CLIENT.set(ws_client.clone()).ok();

                if let Err(e) = register_screenshot_hotkey(&app_handle, ws_client) {
                    eprintln!("Hotkey failed: {}", e);
                    return;
                }
            });

            Ok(())
        })
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                if let Some(ws) = WS_CLIENT.get() {
                    tauri::async_runtime::block_on(async {
                        ws.close().await;
                    });
                }

                if let Some(ocr) = OCR_PROCESS.get() {
                    ocr.stop().ok();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn wait_for_websocket_port(port: u16, timeout_secs: u64) -> bool {
    use tokio_tungstenite::connect_async;

    let url = format!("ws://127.0.0.1:{}", port);

    for _ in 0..timeout_secs {
        sleep(Duration::from_secs(1)).await;

        if let Ok((stream, _)) = connect_async(&url).await {
            drop(stream);
            return true;
        }
    }

    false
}
