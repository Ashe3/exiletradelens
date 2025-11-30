use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

use crate::services::WsClient;

pub fn register_screenshot_hotkey(app: &AppHandle, ws_client: Arc<WsClient>) -> Result<(), String> {
    let shortcut: Shortcut = "Cmd+D"
        .parse()
        .map_err(|e| format!("Invalid shortcut: {:?}", e))?;

    // blocking input while processing image
    let is_processing = Arc::new(AtomicBool::new(false));

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                if is_processing.load(Ordering::Relaxed) {
                    return;
                }

                let ws = Arc::clone(&ws_client);
                let processing_flag = Arc::clone(&is_processing);

                tauri::async_runtime::spawn(async move {
                    processing_flag.store(true, Ordering::Relaxed);
                    handle_screenshot_capture(ws).await.ok();
                    processing_flag.store(false, Ordering::Relaxed);
                });
            }
        })
        .map_err(|e| format!("Failed to register shortcut: {:?}", e))?;

    Ok(())
}

async fn handle_screenshot_capture(ws_client: Arc<WsClient>) -> Result<String, String> {
    let output = tokio::process::Command::new("screencapture")
        .arg("-ic")
        .output()
        .await
        .map_err(|e| format!("screencapture failed: {}", e))?;

    if !output.status.success() {
        return Err("screencapture command failed".to_string());
    }

    // Wait for the clipboard to be populated after the screencapture command.
    // The 200ms delay was chosen empirically and may need adjustment on slower systems
    // or under heavy load. If clipboard errors occur, consider increasing this value.
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    let image_data = read_clipboard_image()?;

    const MAX_SIZE: usize = 5 * 1024 * 1024;
    if image_data.len() > MAX_SIZE {
        return Err("Image too large".to_string());
    }

    #[cfg(debug_assertions)]
    {
        tokio::fs::write("../debug_screenshot.png", &image_data).await.ok();
    }

    ws_client.send_image(image_data).await
}

fn read_clipboard_image() -> Result<Vec<u8>, String> {
    use arboard::Clipboard;
    use image::{ImageBuffer, ImageFormat};
    use std::io::Cursor;

    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard init failed: {}", e))?;

    let img = clipboard
        .get_image()
        .map_err(|e| format!("No image in clipboard: {}", e))?;

    let mut buffer = Cursor::new(Vec::new());
    let image_buffer =
        ImageBuffer::from_raw(img.width as u32, img.height as u32, img.bytes.into_owned())
            .ok_or("Failed to create image buffer")?;

    image::DynamicImage::ImageRgba8(image_buffer)
        .write_to(&mut buffer, ImageFormat::Png)
        .map_err(|e| format!("PNG encoding failed: {}", e))?;

    Ok(buffer.into_inner())
}
