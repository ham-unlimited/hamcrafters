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
    de::{self, SeqAccess, Visitor},
};

/// Clientbound game event packet.
#[derive(Debug)]
#[mc_packet(0x26)]
pub enum GameEvent {
    /// Displays message 'block.minecraft.spawn.not_valid' (You have no home bed or charged respawn anchor, or it was obstructed) to the player.
    NoRespawnBlockAvailable,
    /// Begin raining.
    BeginRaining,
    /// Stop raining.
    EndRaining,
    /// Change the player's game mode.
    ChangeGameMode(GameMode),
    /// What should happen when the player wins the game.
    WinGame(WinGameMode),
    /// Demo event variants.
    DemoEvent(DemoEvent),
    /// Sent when any player was struck by an arrow.
    ArrowHitPlayer,
    /// Change the rain level. (Between 0 and 1).
    RainLevelChange(f32),
    /// Change the thunder level. (Between 0 and 1).
    ThunderLevelChange(f32),
    /// Play pufferfish sting sound.
    PlayPufferfishStingSound,
    /// Play elder guardian mob appearance (effect and sound).
    PlayElderGuardianMobAppearanceSound,
    /// Enable or disable the respawn screen when the player dies or changes dimension.
    EnableRespawnScreen(RespawnScreen),
    /// Enable or disable limited crafting. Sent when the doLimitedCrafting gamerule changes.
    LimitedCrafting(LimitedCraftingMode),
    /// Instructs the client to begin the waiting process for the level chunks.
    /// Sent by the server after the level is cleared on the client and is being re-sent (either during the first, or subsequent reconfigurations).
    StartWaitingForLevelChunks,
}

struct GameEventVisitor;

impl<'de> Visitor<'de> for GameEventVisitor {
    type Value = GameEvent;

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let event_type: u8 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("Expected game event variant byte"))?;
        let value: f32 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom(&format!("Failed to read game event value")))?;

        match event_type {
            0 => Ok(GameEvent::NoRespawnBlockAvailable),
            1 => Ok(GameEvent::BeginRaining),
            2 => Ok(GameEvent::EndRaining),
            3 => {
                let game_mode_value = value.round() as u8;
                let game_mode = GameMode::try_from(game_mode_value)
                    .map_err(|_| de::Error::custom("Invalid game mode value"))?;
                Ok(GameEvent::ChangeGameMode(game_mode))
            }
            4 => Ok(GameEvent::WinGame(if value == 0.0 {
                WinGameMode::JustRespawn
            } else {
                WinGameMode::RollCredits
            })),
            5 => {
                let demo_event_value = value.round() as u8;
                let demo_event = match demo_event_value {
                    0 => DemoEvent::ShowWelcomeScreen,
                    1 => DemoEvent::MovementControls,
                    2 => DemoEvent::JumpControls,
                    3 => DemoEvent::InventoryControls,
                    4 => DemoEvent::DemoOver,
                    _ => return Err(de::Error::custom("Invalid demo event value")),
                };
                Ok(GameEvent::DemoEvent(demo_event))
            }
            6 => Ok(GameEvent::ArrowHitPlayer),
            7 => {
                if value < 0.0 || value > 1.0 {
                    return Err(de::Error::custom("Rain level must be between 0 and 1"));
                }
                Ok(GameEvent::RainLevelChange(value))
            }
            8 => {
                if value < 0.0 || value > 1.0 {
                    return Err(de::Error::custom("Thunder level must be between 0 and 1"));
                }
                Ok(GameEvent::ThunderLevelChange(value))
            }
            9 => Ok(GameEvent::PlayPufferfishStingSound),
            10 => Ok(GameEvent::PlayElderGuardianMobAppearanceSound),
            11 => Ok(GameEvent::EnableRespawnScreen(if value == 0.0 {
                RespawnScreen::EnableRespawnScreen
            } else {
                RespawnScreen::ImmediatelyRespawn
            })),
            12 => Ok(GameEvent::LimitedCrafting(if value == 0.0 {
                LimitedCraftingMode::DisabledLimitedCrafting
            } else {
                LimitedCraftingMode::EnabledLimitedCrafting
            })),
            13 => Ok(GameEvent::StartWaitingForLevelChunks),
            _ => Err(de::Error::custom(format!(
                "Unknown game event type: {event_type}"
            ))),
        }
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid game event packet")
    }
}

impl<'de> Deserialize<'de> for GameEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(GameEventVisitor)
    }
}

/// What should happen when the player wins the game.
#[derive(Debug)]
pub enum WinGameMode {
    /// Just respawn the player without rolling credits.
    JustRespawn,
    /// Only sent by the vanilla client if the player doesn't have the Achievement "The end?".
    RollCredits,
}

/// Demo event variants.
#[derive(Debug)]
pub enum DemoEvent {
    /// Show welcome to demo screen.
    ShowWelcomeScreen,
    /// Tell movement controls
    MovementControls,
    /// Tell jump controls
    JumpControls,
    /// Tell inventory controls
    InventoryControls,
    /// Tell that the demo is over and print a message about how to take a screenshot.
    DemoOver,
}

/// What to do with the respawn screen when the player dies or changes dimension.
#[derive(Debug)]
pub enum RespawnScreen {
    /// Enable respawn screen.
    EnableRespawnScreen,
    /// Immediately respawn (sent when the doImmediateRespawn gamerule changes).
    ImmediatelyRespawn,
}

/// Limited crafting mode variants.
#[derive(Debug)]
pub enum LimitedCraftingMode {
    /// Disable limited crafting.
    DisabledLimitedCrafting,
    /// Enable limited crafting. Sent when the doLimitedCrafting gamerule changes to true.
    EnabledLimitedCrafting,
}
