use std::{
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
};

pub struct OcrService {
    child: Arc<Mutex<Option<Child>>>,
}

impl OcrService {
    pub fn new() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self) -> Result<(), String> {
        let mut child_lock = self.child.lock().unwrap();

        #[cfg(debug_assertions)]
        let (executable, args) = {
            let mut cwd =
                std::env::current_dir().map_err(|e| format!("Failed to get cwd: {}", e))?;

            cwd.pop();

            let python_path = cwd.join("python/venv/bin/python3");
            let script_path = cwd.join("python/ocr_process.py");

            if !python_path.exists() {
                return Err(format!("Python not found: {}", python_path.display()));
            }

            if !script_path.exists() {
                return Err(format!("Script not found: {}", script_path.display()));
            }

            (python_path, vec![script_path])
        };

        #[cfg(not(debug_assertions))]
        let (executable, args) = {
            let exe_dir = std::env::current_exe()
                .map_err(|e| format!("Failed to get exe path: {}", e))?
                .parent()
                .ok_or("Failed to get exe parent dir")?
                .to_path_buf();

            let ocr_binary = exe_dir.join("ocr_process");

            if !ocr_binary.exists() {
                return Err(format!("OCR binary not found: {}", ocr_binary.display()));
            }

            (ocr_binary, vec![])
        };

        let mut command = Command::new(&executable);
        for arg in args {
            command.arg(arg);
        }
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        let child = command
            .spawn()
            .map_err(|e| format!("Failed to spawn OCR: {}", e))?;

        *child_lock = Some(child);
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut child_lock = self.child.lock().unwrap();

        if let Some(mut child) = child_lock.take() {
            child.kill().map_err(|e| format!("Failed to kill: {}", e))?;
            child.wait().ok();
        }

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        let mut child_lock = self.child.lock().unwrap();

        if let Some(child) = child_lock.as_mut() {
            match child.try_wait() {
                Ok(None) => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

impl Drop for OcrService {
    fn drop(&mut self) {
        self.stop().ok();
    }
}
