//! WebSocket communication for Olympus Vanta XRF

mod command;
pub mod error;
pub mod message;
pub mod notification;
pub mod response;
pub mod vanta;

pub use error::{Error, Result};
pub use notification::{Notification, Parameters as NotificationParameters};
pub use response::{Parameters as ResponseParameters, Response};
pub use tungstenite;
pub use vanta::*;
