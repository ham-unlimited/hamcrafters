use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

use crate::codec::var_int::VarInt;

#[derive(Debug, Deserialize)]
#[mc_packet(0x0)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub intent: VarInt, // 1 = Status, 2 = Login, 3 = Transfer(?)
}
