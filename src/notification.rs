//! Helpers for Vanta WebSocket notification deconstruction

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all_fields = "camelCase")]
#[non_exhaustive]
pub enum Parameters {
    None {},
    TestStopped(String, i32),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub command_id: u32,
    pub id: u32,
    pub params: Parameters,
}
