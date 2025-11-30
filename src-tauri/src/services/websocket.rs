use base64::{engine::general_purpose, Engine as _};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct WsClient {
    url: String,
    stream: Mutex<Option<WsStream>>,
}

impl WsClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            stream: Mutex::new(None),
        }
    }

    async fn ensure_connected(&self) -> Result<(), String> {
        let mut stream = self.stream.lock().await;

        if stream.is_none() {
            let (ws_stream, _) = connect_async(&self.url)
                .await
                .map_err(|e| format!("Connection failed: {}", e))?;

            *stream = Some(ws_stream);
        }

        Ok(())
    }

    pub async fn send_image(&self, image_data: Vec<u8>) -> Result<String, String> {
        self.ensure_connected().await?;

        let mut stream_guard = self.stream.lock().await;

        let result = if let Some(stream) = stream_guard.as_mut() {
            let base64_img = general_purpose::STANDARD.encode(image_data);

            if let Err(_) = stream.send(Message::Text(base64_img)).await {
                None
            } else {
                loop {
                    match stream.next().await {
                        Some(Ok(Message::Text(text))) => {
                            break Some(Ok(text));
                        }
                        Some(Ok(Message::Binary(data))) => {
                            break Some(
                                String::from_utf8(data)
                                    .map_err(|e| format!("Invalid UTF-8: {}", e)),
                            );
                        }
                        Some(Ok(Message::Ping(_))) | Some(Ok(Message::Pong(_))) => {
                            continue;
                        }
                        Some(Ok(Message::Close(_))) => {
                            break None;
                        }
                        Some(Ok(_)) | Some(Err(_)) | None => {
                            break None;
                        }
                    }
                }
            }
        } else {
            return Err("No connection".to_string());
        };

        match result {
            Some(Ok(response)) => Ok(response),
            Some(Err(e)) => Err(e),
            None => {
                *stream_guard = None;
                Err("Connection error, will reconnect on next attempt".to_string())
            }
        }
    }

    pub async fn close(&self) {
        let mut stream = self.stream.lock().await;
        if let Some(mut ws) = stream.take() {
            ws.send(Message::Close(None)).await.ok();
        }
    }
}
