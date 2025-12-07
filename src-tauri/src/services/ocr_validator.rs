use serde::{Deserialize, Serialize};

pub struct ValidationError {
    pub message: String,
}

pub struct OcrValidator;

impl OcrValidator {
    pub fn new() -> Self {
        OcrValidator
    }

    pub fn load_and_validate(&self, raw_text: String) {
        let lines: Vec<String> = match serde_json::from_str(&raw_text) {
            Ok(arr) => arr,
            Err(e) => {
                return Err(ValidationError {
                    message: format!("Failed to parse JSON: {}", e),
                });
            }
        };

        if lines.is_empty() {
            return Err(ValidationError {
                message: "Empty OCR result",
            });
        }
    }
}
