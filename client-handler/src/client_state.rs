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
}
