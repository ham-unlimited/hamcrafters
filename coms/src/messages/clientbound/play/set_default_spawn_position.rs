use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::{
        game_mode::{GameMode, PreviousGameMode},
        position::Position,
    },
};
use mc_packet_macros::mc_packet;
use serde::{
    Deserialize, Serialize,
    de::{SeqAccess, Visitor},
};

/// Clientbound set default spawn position packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x5F)]
pub struct SetDefaultSpawnPosition {
    /// Name of the spawn dimension.
    pub dimension_name: Identifier,
    /// The spawn position.
    pub location: Position,
    /// The yaw after respawning.
    pub yaw: f32,
    /// The pitch after respawning.
    pub pitch: f32,
}
