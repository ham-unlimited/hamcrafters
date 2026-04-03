use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Clientbound update entity position packet during play phase.
/// This packet is sent by the server when an entity moves a small distance.
/// The change in position is represented as a fixed-point number with 12 fraction bits and 4 integer bits.
/// As such, the maximum movement distance along each axis is 8 blocks in the negative direction, or 7.999755859375 blocks in the positive direction.
/// If the movement exceeds these limits, Teleport Entity should be sent instead.
#[derive(Debug, Deserialize)]
#[mc_packet(0x33)]
pub struct UpdateEntityPosition {
    /// The ID of the entity to update the position of.
    entity_id: VarInt,
    /// Change in X position as currentX * 4096 - prevX * 4096.
    delta_x: i16,
    /// Change in Y position as currentY * 4096 - prevY * 4096.
    delta_y: i16,
    /// Change in Z position as currentZ * 4096 - prevZ * 4096.
    delta_z: i16,
    /// Whether the entity is on the ground.
    on_ground: bool,
}
