use crate::{
    McPacket,
    codec::{mc_uuid::McUuid, prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::{game_mode::GameMode, text_component::TextComponent},
};
use mc_packet_macros::mc_packet;
use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};
use uuid::Uuid;

/// Clientbound player info update packet.
#[derive(Debug)]
#[mc_packet(0x44)]
pub struct PlayerInfoUpdate {
    /// The actions for each player.
    pub players: PrefixedArray<PlayerInfo>,
}

struct PlayerInfoUpdateVisitor;

impl<'de> Visitor<'de> for PlayerInfoUpdateVisitor {
    type Value = PlayerInfoUpdate;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a player info update packet")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let actions: u8 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("Missing actions field"))?;

        let mut players_count: VarInt = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("Expected players array length"))?;

        let mut players = Vec::new();
        for _ in 0..players_count.0 {
            let player_uuid: McUuid = seq
                .next_element()?
                .ok_or_else(|| de::Error::custom("Expected player UUID"))?;

            let mut player_actions = Vec::new();

            if actions & 0x01 != 0 {
                let add_player_info = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Expected add player info"))?;
                player_actions.push(PlayerAction::AddPlayer(add_player_info));
            }

            if actions & 0x02 != 0 {
                let initialize_chat_info = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Expected initialize chat info"))?;
                player_actions.push(PlayerAction::InitializeChat(initialize_chat_info));
            }

            if actions & 0x04 != 0 {
                let update_gamemode_info = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Expected update gamemode info"))?;
                player_actions.push(PlayerAction::UpdateGamemode(update_gamemode_info));
            }

            if actions & 0x08 != 0 {
                let update_listed_info = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Expected update listed info"))?;
                player_actions.push(PlayerAction::UpdateListed(update_listed_info));
            }

            if actions & 0x10 != 0 {
                let update_latency_info = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Expected update latency info"))?;
                player_actions.push(PlayerAction::UpdateLatency(update_latency_info));
            }

            if actions & 0x20 != 0 {
                let update_display_name_info = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Expected update display name info"))?;
                player_actions.push(PlayerAction::UpdateDisplayName(update_display_name_info));
            }

            if actions & 0x40 != 0 {
                let update_list_priority_info = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Expected update list priority info"))?;
                player_actions.push(PlayerAction::UpdateListPriority(update_list_priority_info));
            }

            if actions & 0x80 != 0 {
                let update_hat_info = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("Expected update hat info"))?;
                player_actions.push(PlayerAction::UpdateHat(update_hat_info));
            }

            players.push(PlayerInfo {
                uuid: player_uuid,
                actions: player_actions,
            });
        }

        Ok(PlayerInfoUpdate {
            players: PrefixedArray::new(players),
        })
    }
}

impl<'de> Deserialize<'de> for PlayerInfoUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(PlayerInfoUpdateVisitor)
    }
}

/// Information about a specific player..
#[derive(Debug)]
pub struct PlayerInfo {
    /// The UUID of the player.
    pub uuid: McUuid,
    /// The actions to update for this player.
    pub actions: Vec<PlayerAction>,
}

/// The specific action to update for a player.
#[derive(Debug)]
pub enum PlayerAction {
    /// Add a player to the player list.
    AddPlayer(AddPlayerInfo),
    /// Initialize the player's chat session.
    InitializeChat(InitializeChatInfo),
    /// Update the player's game mode.
    UpdateGamemode(UpdateGamemodeInfo),
    /// Update whether the player is listed in the player list (TAB).
    UpdateListed(UpdateListedInfo),
    /// Update the player's latency (ping).
    UpdateLatency(UpdateLatencyInfo),
    /// Update the player's display name in the player list (TAB).
    UpdateDisplayName(UpdateDisplayNameInfo),
    /// Update the player's list priority (for sorting in the player list).
    UpdateListPriority(UpdateListPriorityInfo),
    /// Update whether the player's hat is visible.
    UpdateHat(UpdateHatInfo),
}

/// The information to add a player to the player list.
#[derive(Debug, Deserialize)]
pub struct AddPlayerInfo {
    /// The player's name (max 16 bytes).
    pub name: String,
    /// The player's game profile properties.
    pub game_profile_properties: PrefixedArray<GameProfileProperty>,
}

/// A game profile property for a player.
#[derive(Debug, Deserialize)]
pub struct GameProfileProperty {
    /// Name (max 64 bytes)
    pub name: String,
    /// Value (max 32767 bytes)
    pub value: String,
    /// Signature (optional, max 1024 bytes)
    pub signature: Option<String>,
}

/// The information to initialize a player's chat session.
#[derive(Debug, Deserialize)]
pub struct InitializeChatInfo {
    /// The data to update for that player (if present).
    pub data: Option<ChatData>,
}

/// The chat data to initialize a player's chat session.
#[derive(Debug, Deserialize)]
pub struct ChatData {
    /// The session ID for the chat.
    pub chat_session_id: Uuid,
    /// Key expiry time, as a UNIX timestamp in milliseconds. Only sent if Has Signature Data is true.
    pub public_key_expiry_time: i64,
    /// The player's public key, in bytes. Only sent if Has Signature Data is true (512 bytes).
    pub encoded_public_key: PrefixedArray<u8>,
    /// The public key's digital signature. Only sent if Has Signature Data is true (4096 bytes).
    pub public_key_signature: PrefixedArray<u8>,
}

/// The information to update a player's game mode.
#[derive(Debug, Deserialize)]
pub struct UpdateGamemodeInfo {
    /// The player's game mode.
    pub gamemode: GameMode,
}

/// The information to update whether a player is listed in the player list (TAB).
#[derive(Debug, Deserialize)]
pub struct UpdateListedInfo {
    /// Whether the player is listed in the player list (TAB).
    pub listed: bool,
}

/// The information to update a player's latency (ping).
#[derive(Debug, Deserialize)]
pub struct UpdateLatencyInfo {
    /// The player's latency (ping) in milliseconds.
    pub ping: VarInt,
}

/// The information to update a player's display name in the player list (TAB).
#[derive(Debug, Deserialize)]
pub struct UpdateDisplayNameInfo {
    /// The player's display name in the player list (TAB) if present.
    pub display_name: Option<TextComponent>,
}

/// The information to update a player's list priority (for sorting in the player list).
#[derive(Debug, Deserialize)]
pub struct UpdateListPriorityInfo {
    /// The player's list priority (for sorting in the player list).
    pub list_priority: VarInt,
}

/// The information to update whether a player's hat is visible.
#[derive(Debug, Deserialize)]
pub struct UpdateHatInfo {
    /// Whether the player's hat is visible.
    pub visible: bool,
}
