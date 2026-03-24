use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::{
        game_mode::{GameMode, PreviousGameMode},
        position::Position,
    },
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound login packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x3E)]
pub struct PlayerAbilities {
    /// The flags of the player's abilities.
    pub flags: PlayerAbilityFlags,
    /// The fly speed of the player.
    pub fly_speed: f32,
    /// The walk speed of the player.
    pub walk_speed: f32,
}

/// The flags of the player's abilities.
#[derive(Debug)]
pub struct PlayerAbilityFlags {
    /// Whether the player is invulnerable or not.
    pub invulnerable: bool,
    /// Whether the player is flying or not.
    pub flying: bool,
    /// Whether the player can fly or not.
    pub can_fly: bool,
    /// Whether the player is in creative mode or not.
    pub creative_mode: bool,
}

impl<'de> Deserialize<'de> for PlayerAbilityFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let flags = u8::deserialize(deserializer)?;
        Ok(Self {
            invulnerable: flags & 0x01 != 0,
            flying: flags & 0x02 != 0,
            can_fly: flags & 0x04 != 0,
            creative_mode: flags & 0x08 != 0,
        })
    }
}

impl Serialize for PlayerAbilityFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut flags = 0;
        if self.invulnerable {
            flags |= 0x01;
        }
        if self.flying {
            flags |= 0x02;
        }
        if self.can_fly {
            flags |= 0x04;
        }
        if self.creative_mode {
            flags |= 0x08;
        }
        flags.serialize(serializer)
    }
}
