use crate::McPacket;
use crate::codec::var_int::VarInt;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Login packet request message
#[derive(Debug, Deserialize)]
#[mc_packet(0x04)]
pub struct LoginPluginRequest {
    message_id: VarInt,
    channel: String,
    data: Vec<u8>, // length must be infered from the packet length
}
