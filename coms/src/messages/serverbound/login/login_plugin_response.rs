use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

use crate::codec::{prefixed_optional::PrefixedOptional, var_int::VarInt};

/// Login plugin response message
#[derive(Debug, Deserialize, Serialize)]
#[mc_packet(0x02)]
pub struct LoginPluginResponse {
    message_id: VarInt,
    data: PrefixedOptional<Vec<u8>>,
}
