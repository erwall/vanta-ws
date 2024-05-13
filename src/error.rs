//! Error type implementation

use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    SerializeError(#[from] serde_json::Error),
    WebSocketError(#[from] tungstenite::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SerializeError(e) => write!(f, "{e}"),
            Error::WebSocketError(e) => write!(f, "{e}"),
        }
    }
}
