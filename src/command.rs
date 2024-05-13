//! Helpers for Vanta WebSocket command construction

use serde::{Deserialize, Serialize};
use tungstenite::Message;

const ON_CONNECT_RESPONSE: &str = "Hi i am a controller";

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

pub fn construct_msg_login(
    id: u32,
    user_id: String,
    password: String,
) -> serde_json::Result<Message> {
    let params = VantaCommandParameters::Login(VantaParametersLogin { user_id, password });
    construct_binary_message(301, id, params)
}

pub fn construct_msg_logout(id: u32) -> serde_json::Result<Message> {
    construct_text_message(319, id, VantaCommandParameters::None)
}

pub fn construct_msg_clear_faults(id: u32) -> serde_json::Result<Message> {
    construct_text_message(254, id, VantaCommandParameters::None)
}

pub fn construct_msg_start_test(id: u32) -> serde_json::Result<Message> {
    construct_text_message(601, id, VantaCommandParameters::None)
}

pub fn construct_msg_pet_watchdog(id: u32) -> serde_json::Result<Message> {
    construct_text_message(244, id, VantaCommandParameters::None)
}

pub fn construct_on_connect_response() -> Message {
    Message::Text(String::from(ON_CONNECT_RESPONSE))
}

pub fn construct_binary_message(
    command_id: u32,
    id: u32,
    params: VantaCommandParameters,
) -> serde_json::Result<Message> {
    let command = VantaCommand {
        command_id,
        id,
        params,
    };
    let message = serde_json::to_vec(&command)?;
    Ok(Message::Binary(message))
}

pub fn construct_text_message(
    command_id: u32,
    id: u32,
    params: VantaCommandParameters,
) -> serde_json::Result<Message> {
    let command = VantaCommand {
        command_id,
        id,
        params,
    };
    let message = serde_json::to_string(&command)?;
    Ok(Message::Text(message))
}
