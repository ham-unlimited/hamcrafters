use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound login packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x30)]
pub struct Login {
    /// The entity ID of the player.
    pub entity_id: i32,
    /// Whether the player is in hardcore mode.
    pub is_hardcore: bool,
    /// List of dimensions on the server.
    pub dimension_names: PrefixedArray<Identifier>,
    /// The maximum number of players allowed on the server (now ignored).
    pub max_players: VarInt,
    /// View distance on the server (in chunks).
    pub view_distance: VarInt,
    /// The simulation distance on the server (in chunks).
    pub simulation_distance: VarInt,
    /// Whether the server is in reduced debug info mode.
    pub reduced_debug_info: bool,
    /// Whether the server is in immediate respawn or not.
    pub enable_respawn_screen: bool,
    /// Whether the do_limited_crafting gamerule is enabled on the server or not.
    pub do_limited_crafting: bool,
    /// The ID of the dimension the player is in (refer to minecraft:dimension_type registry).
    pub dimension_type: VarInt,
    /// The name of the dimension the player is in.
    pub dimension_name: Identifier,
    /// The hashed seed of the world, specifically the first 8 bytes of the SHA-256 of the world's seed.
    pub hashed_seed: i64,
    /// The current game mode of the player. (0 = Survival, 1 = Creative, 2 = Adventure, 3 = Spectator)
    pub game_mode: u8,
    /// The previous game mode of the player. (-1 = undefined, 0 = Survival, 1 = Creative, 2 = Adventure, 3 = Spectator)
    pub previous_game_mode: i8,
    /// Whether the player is in a debug world.
    pub is_debug: bool,
    /// Whether the player is in a flat world.
    pub is_flat: bool,
    /// The death location of the player, if the server has one.
    pub death_location: Option<DeathLocation>,
    /// The portal cooldown of the player, in ticks.
    pub portal_cooldown: VarInt,
    /// The sea level of the world.
    pub sea_level: VarInt,
    /// Whether the server enforces secure chat or not.
    pub enforces_secure_chat: bool,
}

/// The location of a player's death.
#[derive(Debug, Deserialize)]
pub struct DeathLocation {
    /// The name of the dimension the player died in.
    pub dimension_name: Identifier,
    /// The coordinates of the player's death location.
    pub position: Position,
}

// TODO: MOVE SOMEWHERE MORE RELEVANT.
// TODO: Test this? E.g. 0100011000000111011000110010110000010101101101001000001100111111 should be x = 18357644, y = 831 and z = -20882616.
/// Represents a position in the Minecraft world.
#[derive(Debug)]
pub struct Position {
    /// The X coordinate of the position (26-bit).
    pub x: i32,
    /// The Y coordinate of the position (12-bit).
    pub y: i16,
    /// The Z coordinate of the position (26-bit).
    pub z: i32,
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;

        let x = (value >> 38) as i32;
        let y = (value << 52 >> 52) as i16;
        let z = (value << 26 >> 38) as i32;
        Ok(Position { x, y, z })
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = ((self.x as i64 & 0x3FFFFFF) << 38)
            | ((self.z as i64 & 0x3FFFFFF) << 12)
            | (self.y as i64 & 0xFFF);
        value.serialize(serializer)
    }
}
