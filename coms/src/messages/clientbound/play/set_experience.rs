use crate::{McPacket, codec::var_int::VarInt};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound set health packet.
/// Sent by the server when the client should change experience levels.
#[derive(Debug, Deserialize)]
#[mc_packet(0x65)]
pub struct SetExperience {
    /// Between 0 and 1.
    pub experience_bar: f32,
    /// Level? (Not entirely sure).
    pub level: VarInt,
    /// Total experience points? (Not entirely sure if this is a delta or the total).
    pub total_experience: VarInt,
}
