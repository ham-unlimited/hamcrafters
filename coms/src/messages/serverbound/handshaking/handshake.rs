use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

use crate::codec::var_int::VarInt;

/// A Minecraft Handshake packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x0)]
pub struct Handshake {
    /// The protocol version of the client requesting the handshake.
    pub protocol_version: VarInt,
    /// The server address the client used to connect to this server.
    pub server_address: String,
    /// The server port the client used to connect to this server.
    pub server_port: u16,
    /// Which state the client wishes this connection to enter.
    /// 1: Status
    /// 2: Login
    /// 3: Transfer (no idea).
    pub intent: VarInt,
}
