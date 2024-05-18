//! WebSocket communication for Olympus Vanta XRF

mod command;
pub mod error;
pub mod message;
pub mod vanta;

pub use error::{Error, Result};
pub use tungstenite;
pub use vanta::*;
