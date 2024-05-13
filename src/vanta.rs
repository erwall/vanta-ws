//! Functions for a client communicating with Vanta WebSocket server

use crate::command::*;
use crate::error::Error;
use std::io::{Read, Write};
use tungstenite::WebSocket;

const CONNECTION_ESTABLISHED: &str = "Server ready to receive commands";
const CONNECTION_REJECTED_DEVICE_CONTROLLED: &str = "Rejecting connection. Device being controlled";
const CONNECTION_REJECTED_USER_LOGGED_IN: &str = "Rejecting connection. User is Logged In";

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
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(construct_msg_login(id, user_id, password)?)?;
    Ok(socket)
}

pub fn logout<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(construct_msg_logout(id)?)?;
    Ok(socket)
}

pub fn clear_faults<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(construct_msg_clear_faults(id)?)?;
    Ok(socket)
}

pub fn start_test<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(construct_msg_start_test(id)?)?;
    Ok(socket)
}

pub fn pet_watchdog<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, Error> {
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
