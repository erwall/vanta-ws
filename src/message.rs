use crate::command::{self, Command};
use tungstenite::Message;

const CONNECTION_ESTABLISHED: &str = "Server ready to receive commands";
const CONNECTION_REJECTED_DEVICE_CONTROLLED: &str = "Rejecting connection. Device being controlled";
const CONNECTION_REJECTED_USER_LOGGED_IN: &str = "Rejecting connection. User is Logged In";
const ON_CONNECT_RESPONSE: &str = "Hi i am a controller";

#[derive(Debug, PartialEq)]
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

impl From<&Message> for VantaState {
    fn from(value: &Message) -> Self {
        match value {
            Message::Text(s) => VantaState::from(s),
            _ => VantaState::Unknown,
        }
    }
}

fn construct_binary_message(
    command_id: u32,
    id: u32,
    params: Option<command::Parameters>,
) -> serde_json::Result<Message> {
    let command = Command {
        command_id,
        id,
        params,
    };
    let message = serde_json::to_vec(&command)?;
    Ok(Message::Binary(message))
}

pub fn login(id: u32, user_id: String, password: String) -> crate::Result<Message> {
    let params = command::Parameters::Login { user_id, password };
    Ok(construct_binary_message(301, id, Some(params))?)
}

pub fn logout(id: u32) -> crate::Result<Message> {
    Ok(construct_binary_message(319, id, None)?)
}

pub fn clear_faults(id: u32) -> crate::Result<Message> {
    Ok(construct_binary_message(254, id, None)?)
}

pub fn start_test(id: u32) -> crate::Result<Message> {
    Ok(construct_binary_message(601, id, None)?)
}

pub fn pet_watchdog(id: u32) -> crate::Result<Message> {
    Ok(construct_binary_message(244, id, None)?)
}

pub fn on_connect_response() -> crate::Result<Message> {
    Ok(Message::Text(String::from(ON_CONNECT_RESPONSE)))
}
