use std::fmt::Display;

use owo_colors::OwoColorize;

/// The states that the client-server coms can be in.
#[derive(Debug, Clone, Default)]
pub enum ClientState {
    /// Handshaking state
    #[default]
    Handshaking,
    /// Status state, used for the server status page (Multiplayer server list).
    Status,
    /// Login state.
    Login,
    /// Configuration state (after login).
    Configuration,
    /// During gameplay.
    Play,
}

impl Display for ClientState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ClientState::Handshaking => "Handshaking".bright_blue().to_string(),
                ClientState::Status => "Status".bright_green().to_string(),
                ClientState::Login => "Login".yellow().to_string(),
                ClientState::Configuration => "Configuration".red().to_string(),
                ClientState::Play => "Play".green().to_string(),
            }
        )
    }
}
