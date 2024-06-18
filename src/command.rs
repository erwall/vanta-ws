//! Helpers for Vanta WebSocket command construction

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all_fields = "camelCase")]
pub enum Parameters {
    Login { user_id: String, password: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    pub command_id: u32,
    pub id: u32,
    #[serde(default)]
    pub params: Option<Parameters>,
}
