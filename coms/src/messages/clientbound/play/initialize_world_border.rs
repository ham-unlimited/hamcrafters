use crate::{
    McPacket,
    codec::{
        prefixed_array::PrefixedArray, prefixed_optional::PrefixedOptional, var_int::VarInt,
        var_long::VarLong,
    },
    messages::models::text_component::TextComponent,
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Client-bound initialize world border packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x2A)]
pub struct InitializeWorldBorder {
    /// The center of the world border (X coordinate).
    pub x: f64,
    /// The center of the world border (Z coordinate).
    pub z: f64,
    /// Current length of a single side of the world border, in meters.
    pub old_diameter: f64,
    /// Target length of a single side of the world border, in meters.
    pub new_diameter: f64,
    /// Number of real-time milliseconds until New Diameter is reached.
    /// It appears that vanilla server does not sync world border speed to game ticks, so it gets out of sync with server lag.
    /// If the world border is not moving, this is set to 0.
    pub speed: VarLong,
    /// Resulting coordinates from a portal teleport is limited to +- this value. Usually 29999984.
    pub portal_boundary: VarInt,
    /// In meters.
    pub warning_blocks: VarInt,
    /// In seconds, as set by /worldborder warning time.
    pub warning_time: VarInt,
}
