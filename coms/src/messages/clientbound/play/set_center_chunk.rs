use crate::{McPacket, codec::var_int::VarInt};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound set center chunk packet.
/// The vanilla server sends this packet whenever the player moves across a chunk border horizontally, and also (according to testing) for any integer change in the vertical axis, even if it doesn't go across a chunk section border.
#[derive(Debug, Deserialize)]
#[mc_packet(0x5C)]
pub struct SetCenterChunk {
    /// Chunk X coordinate of the loading area center.
    pub chunk_x: VarInt,
    /// Chunk Z coordinate of the loading area center.
    pub chunk_z: VarInt,
}
