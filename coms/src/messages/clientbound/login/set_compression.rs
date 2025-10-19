use crate::McPacket;
use crate::codec::var_int::VarInt;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Set compression message
#[derive(Debug, Deserialize)]
#[mc_packet(0x03)]
pub struct SetCompression {
    threshold: VarInt,
}
