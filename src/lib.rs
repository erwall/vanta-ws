//! WebSocket communication for Olympus Vanta XRF

pub mod error;

use error::VantaError;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use tungstenite::{Message, WebSocket};

const CONNECTION_ESTABLISHED: &str = "Server ready to receive commands";
const CONNECTION_REJECTED_DEVICE_CONTROLLED: &str = "Rejecting connection. Device being controlled";
const CONNECTION_REJECTED_USER_LOGGED_IN: &str = "Rejecting connection. User is Logged In";
const ON_CONNECT_RESPONSE: &str = "Hi i am a controller";

#[derive(Serialize, Deserialize, Debug)]
struct VantaParametersLogin {
    #[serde(rename = "userId")]
    user_id: String,
    password: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
enum VantaCommandParameters {
    #[default]
    None,
    Login(VantaParametersLogin),
}

#[derive(Serialize, Deserialize, Debug)]
struct VantaCommand {
    #[serde(rename = "commandId")]
    command_id: u32,
    id: u32,
    #[serde(default, flatten)]
    params: VantaCommandParameters,
}

pub enum VantaState {
    Disconnected,
    Disconnecting,
    Connecting,
    Ready,
}

pub fn login<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
    user_id: String,
    password: String,
) -> Result<&mut WebSocket<S>, VantaError> {
    socket.write(construct_msg_login(id, user_id, password)?)?;
    Ok(socket)
}

pub fn logout<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, VantaError> {
    socket.write(construct_msg_logout(id)?)?;
    Ok(socket)
}

pub fn clear_faults<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, VantaError> {
    socket.write(construct_msg_clear_faults(id)?)?;
    Ok(socket)
}

pub fn start_test<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, VantaError> {
    socket.write(construct_msg_start_test(id)?)?;
    Ok(socket)
}

pub fn pet_watchdog<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, VantaError> {
    socket.write(construct_msg_pet_watchdog(id)?)?;
    Ok(socket)
}

pub fn get_state_from_message(message: &String) -> VantaState {
    match message.as_str() {
        s if s.contains(CONNECTION_ESTABLISHED) => VantaState::Ready,
        s if s.contains(CONNECTION_REJECTED_DEVICE_CONTROLLED) => VantaState::Disconnecting,
        s if s.contains(CONNECTION_REJECTED_USER_LOGGED_IN) => VantaState::Disconnecting,
        _ => VantaState::Disconnecting,
    }
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

fn construct_binary_message(
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

fn construct_text_message(
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
