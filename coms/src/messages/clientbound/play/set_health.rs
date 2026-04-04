use crate::{McPacket, codec::var_int::VarInt};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound set health packet.
/// Sent by the server to set the health of the player it is sent to.
/// Food saturation acts as a food “overcharge”.
/// Food values will not decrease while the saturation is over zero.
/// New players logging in or respawning automatically get a saturation of 5.0.
/// Eating food increases the saturation as well as the food bar.
#[derive(Debug, Deserialize)]
#[mc_packet(0x66)]
pub struct SetHealth {
    /// The player's health, 0 or less = dead, 20 = full HP.
    pub health: f32,
    /// The player's food level, 0-20.
    pub food: VarInt,
    /// The player's food saturation level, seems to vary from 0.0 to 5.0 in integer increments.
    pub food_saturation: f32,
}
