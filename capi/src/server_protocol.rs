//! The server protocol is an experimental JSON based protocol that is used to
//! remotely control the timer. Every command that you send has a response in
//! the form of a JSON object indicating whether the command was successful or
//! not.

use livesplit_core::networking::server_protocol;
use wasm_bindgen::prelude::*;

use crate::command_sink::CommandSink;

/// The server protocol is an experimental JSON based protocol that is used to
/// remotely control the timer. Every command that you send has a response in
/// the form of a JSON object indicating whether the command was successful or
/// not.
#[wasm_bindgen]
pub struct ServerProtocol {}

#[wasm_bindgen]
impl ServerProtocol {
    /// Handles an incoming command and returns the response to be sent.
    pub async unsafe fn handleCommand(command: &str, commandSink: *const CommandSink) -> String {
        // SAFETY: The caller must ensure that the pointer is valid.
        server_protocol::handle_command(command, unsafe { &*commandSink }).await
    }

    /// Encodes an event that happened to be sent.
    pub fn encodeEvent(event: u32) -> Option<String> {
        Some(server_protocol::encode_event(event.try_into().ok()?))
    }
}
