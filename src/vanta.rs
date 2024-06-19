//! Functions for a client communicating with Vanta WebSocket server

use crate::message::{self, MessageInbound};
use std::io::{Read, Write};
use tungstenite::{Message, WebSocket};

pub fn read<S: Read + Write>(
    socket: &mut WebSocket<S>,
) -> crate::Result<Option<MessageInbound>> {
    let message = match socket.read()? {
        Message::Binary(m) => Some(message::deconstruct_binary_message(m.as_slice())?),
        _ => None,
    };
    Ok(message)
}

pub fn login<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
    user_id: String,
    password: String,
) -> crate::Result<&mut WebSocket<S>> {
    socket.write(message::login(id, user_id, password)?)?;
    Ok(socket)
}

pub fn logout<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> crate::Result<&mut WebSocket<S>> {
    socket.write(message::logout(id)?)?;
    Ok(socket)
}

pub fn clear_faults<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> crate::Result<&mut WebSocket<S>> {
    socket.write(message::clear_faults(id)?)?;
    Ok(socket)
}

pub fn start_test<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> crate::Result<&mut WebSocket<S>> {
    socket.write(message::start_test(id)?)?;
    Ok(socket)
}

pub fn pet_watchdog<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> crate::Result<&mut WebSocket<S>> {
    socket.write(message::pet_watchdog(id)?)?;
    Ok(socket)
}

pub fn on_connect_response<S: Read + Write>(
    socket: &mut WebSocket<S>,
) -> crate::Result<&mut WebSocket<S>> {
    socket.write(message::on_connect_response()?)?;
    Ok(socket)
}
