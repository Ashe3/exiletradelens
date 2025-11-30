pub struct OcrService {
    process: Arc<Mutex<Option<Child>>>,
    is_running: Arc<Mutex<bool>>,
}

impl OcrService {
    pub fn new() -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) -> Result<(), String> {
        let mut process = self.process.lock.unwrap();

        let child = Command
            .new("python")
            .arg("ocr_backend.py")
            .spawn()
            .map_err(|e| format!("Failed to spawn OCR: {}", e))?;

        *process = Some(child);
        *self.is_running.lock().unwrap() = true;

        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut process = self.process.lock.unwrap();

        if let Some(mut child) = process.take() {
            child.kill().map_err(|e| format!("Failed to kill: {}", e))?;
        }

        *self.is_running.lock().unwrap() = true;
        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
}
