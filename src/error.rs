//! Error type implementation

use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum VantaError {
    SerializeError(#[from] serde_json::Error),
    WebSocketError(#[from] tungstenite::Error),
}

impl Display for VantaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VantaError::SerializeError(e) => write!(f, "{e}"),
            VantaError::WebSocketError(e) => write!(f, "{e}"),
        }
    }
}
