//! WebSocket communication for Olympus Vanta XRF

mod command;
pub mod error;
pub mod vanta;

pub use error::Error;
pub use vanta::*;
