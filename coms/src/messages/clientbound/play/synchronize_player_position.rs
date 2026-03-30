use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::{id_set::IdSet, slot_display::SlotDisplay},
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound synchronize player position packet, used to teleport the player (or during login).
#[derive(Debug, Deserialize)]
#[mc_packet(0x46)]
pub struct SynchronizePlayerPosition {
    /// A unique ID for the player teleportation, used to confirm the teleportation.
    pub teleport_id: VarInt,
    /// The X coordinate of the player's new position.
    pub x: f64,
    /// The Y coordinate of the player's new position.
    pub y: f64,
    /// The Z coordinate of the player's new position.
    pub z: f64,
    /// The velocity of the player after teleportation in the x axis.
    pub velocity_x: f64,
    /// The velocity of the player after teleportation in the y axis.
    pub velocity_y: f64,
    /// The velocity of the player after teleportation in the z axis.
    pub velocity_z: f64,
    /// The yaw of the player's new position.
    pub yaw: f32,
    /// The pitch of the player's new position.
    pub pitch: f32,
    /// The flags for the teleportation, used to indicate which coordinates are relative.
    pub flags: TeleportFlags,
}

/// The flags for the teleportation, used to indicate which coordinates are relative.
#[derive(Debug)]
pub struct TeleportFlags {
    /// Whether the X coordinate is relative to the player's current position.
    pub relative_x: bool,
    /// Whether the Y coordinate is relative to the player's current position.
    pub relative_y: bool,
    /// Whether the Z coordinate is relative to the player's current position.
    pub relative_z: bool,
    /// Whether the yaw is relative to the player's current yaw.
    pub relative_yaw: bool,
    /// Whether the pitch is relative to the player's current pitch.
    pub relative_pitch: bool,
    /// Whether the velocity in the x axis is relative to the player's current velocity in the x axis.
    pub relative_velocity_x: bool,
    /// Whether the velocity in the y axis is relative to the player's current velocity in the y axis.
    pub relative_velocity_y: bool,
    /// Whether the velocity in the z axis is relative to the player's current velocity in the z axis.
    pub relative_velocity_z: bool,
    /// Rotate velocity according to the change in rotation, before applying the velocity change in this packet. Combining this with absolute rotation works as expected—the difference in rotation is still used.
    // TODO: Wtf do you call this field?
    pub extra: bool,
}

impl<'de> Deserialize<'de> for TeleportFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let flags = i32::deserialize(deserializer)?;
        Ok(Self {
            relative_x: flags & 0x01 != 0,
            relative_y: flags & 0x02 != 0,
            relative_z: flags & 0x04 != 0,
            relative_yaw: flags & 0x08 != 0,
            relative_pitch: flags & 0x10 != 0,
            relative_velocity_x: flags & 0x20 != 0,
            relative_velocity_y: flags & 0x40 != 0,
            relative_velocity_z: flags & 0x80 != 0,
            extra: flags & 0x100 != 0,
        })
    }
}
