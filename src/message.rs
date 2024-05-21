use crate::command::*;
use tungstenite::Message;

const CONNECTION_ESTABLISHED: &str = "Server ready to receive commands";
const CONNECTION_REJECTED_DEVICE_CONTROLLED: &str = "Rejecting connection. Device being controlled";
const CONNECTION_REJECTED_USER_LOGGED_IN: &str = "Rejecting connection. User is Logged In";
const ON_CONNECT_RESPONSE: &str = "Hi i am a controller";

pub enum VantaState {
    Disconnected,
    Disconnecting,
    Ready,
    Unknown,
}

impl From<&str> for VantaState {
    fn from(value: &str) -> Self {
        match value {
            s if s.contains(CONNECTION_ESTABLISHED) => VantaState::Ready,
            s if s.contains(CONNECTION_REJECTED_DEVICE_CONTROLLED) => VantaState::Disconnecting,
            s if s.contains(CONNECTION_REJECTED_USER_LOGGED_IN) => VantaState::Disconnecting,
            _ => VantaState::Unknown,
        }
    }
}

impl From<&String> for VantaState {
    fn from(value: &String) -> Self {
        VantaState::from(value.as_str())
    }
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


pub fn login(id: u32, user_id: String, password: String) -> serde_json::Result<Message> {
    let params = VantaCommandParameters::Login(VantaParametersLogin { user_id, password });
    construct_binary_message(301, id, params)
}

pub fn logout(id: u32) -> serde_json::Result<Message> {
    construct_text_message(319, id, VantaCommandParameters::None)
}

pub fn clear_faults(id: u32) -> serde_json::Result<Message> {
    construct_text_message(254, id, VantaCommandParameters::None)
}

pub fn start_test(id: u32) -> serde_json::Result<Message> {
    construct_text_message(601, id, VantaCommandParameters::None)
}

pub fn pet_watchdog(id: u32) -> serde_json::Result<Message> {
    construct_text_message(244, id, VantaCommandParameters::None)
}

pub fn on_connect_response() -> Message {
    Message::Text(String::from(ON_CONNECT_RESPONSE))
}
