use crate::{
    McPacket,
    codec::{mc_uuid::McUuid, var_int::VarInt},
    messages::models::{angle::Angle, velocity::Velocity},
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound spawn entity packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x01)]
pub struct SpawnEntity {
    /// unique integer ID mostly used in the protocol to identify the entity.
    /// If an entity with the same ID already exists on the client, it is automatically deleted and replaced by the new entity.
    /// On the vanilla server entity IDs are globally unique across all dimensions and never reused while the server is running, but not preserved across server restarts.
    pub entity_id: VarInt,
    /// A unique identifier that is mostly used in persistence and places where the uniqueness matters more.
    /// It is possible to create multiple entities with the same UUID on the vanilla client, but a warning will be logged, and functionality dependent on UUIDs may ignore the entity or otherwise misbehave.
    pub entity_uuid: McUuid,
    /// ID in the minecraft:entity_type registry.
    pub entity_type: VarInt,
    /// The X coordinate of the entity.
    pub x: f64,
    /// The Y coordinate of the entity.
    pub y: f64,
    /// The Z coordinate of the entity.
    pub z: f64,
    /// The velocity of the entity.
    pub velocity: Velocity,
    /// The pitch of the entity.
    pub pitch: Angle,
    /// The yaw of the entity.
    pub yaw: Angle,
    /// The head yaw of the entity. Only used by living entities, where the head of the entity may differ from the general body rotation.
    pub head_yaw: Angle,
    /// Meaning dependent on the value of the [entity_type] field.
    pub data: VarInt,
}
