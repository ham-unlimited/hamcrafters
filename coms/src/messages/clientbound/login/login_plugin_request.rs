use crate::McPacket;
use crate::codec::var_int::VarInt;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[mc_packet(0x04)]
pub struct LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: String,
    pub data: Vec<u8>, // length must be infered from the packet length
}
