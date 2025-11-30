use crate::services::OcrService;
use tauri::Manager;

mod services;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let ocr_process = OcrService::new();
            ocr_process.start()?;

            app.manage(ocr_process);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
