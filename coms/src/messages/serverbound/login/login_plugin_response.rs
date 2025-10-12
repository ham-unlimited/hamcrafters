use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

use crate::codec::{prefixed_optional::PrefixedOptional, var_int::VarInt};

#[derive(Debug, Deserialize)]
#[mc_packet(0x02)]
pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub data: PrefixedOptional<Vec<u8>>,
}
