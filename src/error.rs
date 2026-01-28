use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("WebSocket connection error: {0}")]
    WebSocketConnection(String),

    #[error("WebSocket send error: {0}")]
    WebSocketSend(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Parse error: {0}")]
    Parse(String),
}
