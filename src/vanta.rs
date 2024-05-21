//! Functions for a client communicating with Vanta WebSocket server

use crate::message;
use crate::error::Error;
use std::io::{Read, Write};
use tungstenite::WebSocket;

pub fn login<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
    user_id: String,
    password: String,
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(message::login(id, user_id, password)?)?;
    Ok(socket)
}

pub fn logout<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(message::logout(id)?)?;
    Ok(socket)
}

pub fn clear_faults<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(message::clear_faults(id)?)?;
    Ok(socket)
}

pub fn start_test<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(message::start_test(id)?)?;
    Ok(socket)
}

pub fn pet_watchdog<S: Read + Write>(
    socket: &mut WebSocket<S>,
    id: u32,
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(message::pet_watchdog(id)?)?;
    Ok(socket)
}

pub fn on_connect_response<S: Read + Write>(
    socket: &mut WebSocket<S>,
) -> Result<&mut WebSocket<S>, Error> {
    socket.write(message::on_connect_response())?;
    Ok(socket)
}
