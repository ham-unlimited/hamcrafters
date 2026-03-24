use serde::{Deserialize, Serialize};

/// A minecraft game mode.
#[derive(Debug)]
pub enum GameMode {
    /// Survival mode.
    Survival,
    /// Creative mode.
    Creative,
    /// Adventure mode.
    Adventure,
    /// Spectator mode.
    Spectator,
}

impl TryFrom<u8> for GameMode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GameMode::Survival),
            1 => Ok(GameMode::Creative),
            2 => Ok(GameMode::Adventure),
            3 => Ok(GameMode::Spectator),
            _ => Err(format!("Invalid game mode value: {value}")),
        }
    }
}

impl<'de> Deserialize<'de> for GameMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        let game_mode = GameMode::try_from(value).map_err(serde::de::Error::custom)?;
        Ok(game_mode)
    }
}

impl Serialize for GameMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: u8 = match self {
            GameMode::Survival => 0,
            GameMode::Creative => 1,
            GameMode::Adventure => 2,
            GameMode::Spectator => 3,
        };
        value.serialize(serializer)
    }
}

/// The previous game mode of the player. (-1 = undefined, 0 = Survival, 1 = Creative, 2 = Adventure, 3 = Spectator)
#[derive(Debug)]
pub enum PreviousGameMode {
    /// The previous game mode is undefined.
    Undefined,
    /// The previous game mode is defined.
    Mode(GameMode),
}

impl<'de> Deserialize<'de> for PreviousGameMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i8::deserialize(deserializer)?;

        match value {
            -1 => Ok(PreviousGameMode::Undefined),
            v => {
                let game_mode = GameMode::try_from(v as u8).map_err(serde::de::Error::custom)?;
                Ok(PreviousGameMode::Mode(game_mode))
            }
        }
    }
}
