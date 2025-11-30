use std::sync::Arc;

use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

use crate::services::WsClient;

pub fn register_screenshot_hotkey(app: &AppHandle, ws_client: Arc<WsClient>) -> Result<(), String> {
    let app_clone = app.clone();
    let ws_clone = ws_client.clone();

    let shortcut: Shortcut = "Cmd+D"
        .parse()
        .map_err(|e| format!("Invalid shortcut: {:?}", e))?;

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, shortcut, event| {
            if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                println!("Hotkey pressed: {:?}", shortcut);

                let app_handle = app_clone.clone();
                let ws = Arc::clone(&ws_clone);

                tauri::async_runtime::spawn(async move {
                    match handle_screenshot_capture(&app_handle, ws).await {
                        Ok(result) => {
                            println!("OCR result: {}", result);
                            app_handle.emit("ocr-completed", result).ok();
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            app_handle.emit("ocr-error", e).ok();
                        }
                    }
                });
            }
        })
        .map_err(|e| format!("Failed to register shortcut: {:?}", e))?;

    Ok(())
}

async fn handle_screenshot_capture(
    app: &AppHandle,
    ws_client: Arc<WsClient>,
) -> Result<String, String> {
    println!("Capturing screenshot...");
    let output = tokio::process::Command::new("screencapture")
        .arg("-ic")
        .output()
        .await
        .map_err(|e| format!("screencapture failed: {}", e))?;

    if !output.status.success() {
        return Err("screencapture command failed".to_string());
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    println!("Reading from clipboard...");
    let image_data = read_clipboard_image()?;

    #[cfg(debug_assertions)]
    {
        tokio::fs::write("debug_screenshot.png", &image_data)
            .await
            .map_err(|e| format!("Save failed: {}", e))?;
        println!("Saved debug_screenshot.png ({} bytes)", image_data.len());
    }

    println!("Sending to OCR...");
    let result = ws_client.send_image(image_data).await?;

    Ok(result)
}

fn read_clipboard_image() -> Result<Vec<u8>, String> {
    use arboard::Clipboard;
    use image::{ImageBuffer, ImageOutputFormat};
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
        .write_to(&mut buffer, ImageOutputFormat::Png)
        .map_err(|e| format!("PNG encoding failed: {}", e))?;

    Ok(buffer.into_inner())
}
