use serde::Deserialize;

use crate::{codec::var_int::VarInt, messages::McPacket};

#[derive(Debug, Deserialize)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub intent: VarInt, // 1 = Status, 2 = Login, 3 = Transfer(?)
}

impl McPacket for Handshake {
    fn packet_id() -> &'static usize {
        &0x0
    }
}
