use crate::{McPacket, codec::var_int::VarInt};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound set held item packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x67)]
pub struct SetHeldItem {
    /// The slot that the player is now holding (0-8).
    pub slot: VarInt,
}
