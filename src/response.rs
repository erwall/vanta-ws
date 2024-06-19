//! Helpers for Vanta WebSocket response deconstruction

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all_fields = "camelCase")]
pub enum Parameters {
    None {},
    Login {
        headless: bool,
        language: String,
        password: String,
        user_id: String,
        user_type: u32,
        with_sound: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginError {
    pub error_code: i32,
    pub error_string: String,
    pub error_type: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub command_id: u32,
    pub id: u32,
    pub params: Parameters,
    pub error: Option<LoginError>,
}
