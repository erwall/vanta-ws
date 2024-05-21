//! Helpers for Vanta WebSocket command construction

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct VantaParametersLogin {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub enum VantaCommandParameters {
    #[default]
    None,
    Login(VantaParametersLogin),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VantaCommand {
    #[serde(rename = "commandId")]
    pub command_id: u32,
    pub id: u32,
    #[serde(default, flatten)]
    pub params: VantaCommandParameters,
}
