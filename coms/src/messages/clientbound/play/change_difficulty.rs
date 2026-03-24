use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

use crate::messages::models::difficulty::Difficulty;

/// Clientbound change difficulty packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x0A)]
pub struct ChangeDifficulty {
    /// The new difficulty level.
    pub difficulty: Difficulty,
    /// Whether the difficulty is locked or not.
    pub difficulty_locked: bool,
}
