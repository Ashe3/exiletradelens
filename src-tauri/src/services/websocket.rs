use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct WsClient {
    url: String,
}

impl WsClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub async fn send_image(&self, image_data: Vec<u8>) -> Result<String, String> {
        let (ws_stream, _) = connect_async(&self.url)
            .await
            .map_err(|e| format!("WS connect failed: {}", e))?;

        let (mut write, mut read) = ws_stream.split();

        let base64_img = base64::encode(image_data);

        write
            .send(Message::Text(base64_img))
            .await
            .map_err(|e| format!("Send failed: {}", e))?;

        if let Some(Ok(Message::Text(response))) = read.next().await {
            Ok(response)
        } else {
            Err("No response from OCR".to_string())
        }
    }
}
