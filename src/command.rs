//! Helpers for Vanta WebSocket command construction

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginParameters {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Parameters {
    #[default]
    None,
    Login(LoginParameters),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    #[serde(rename = "commandId")]
    pub command_id: u32,
    pub id: u32,
    #[serde(default, flatten)]
    pub params: Parameters,
}
