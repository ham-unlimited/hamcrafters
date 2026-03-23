use serde::{Deserialize, Serialize};

use crate::client_state::ClientState;

/// The direction of a packet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketDirection {
    /// Client -> Server
    ClientBound,
    /// Server -> Client
    ServerBound,
}

/// A recorded packet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedPacket {
    /// Milliseconds elapsed since the start of the session
    pub timestamp_ms: u64,
    /// Which direction this packet was traveling
    pub direction: PacketDirection,
    /// Protocol state at the time of this packet
    pub state: ClientState,
    /// Packet ID
    pub packet_id: i32,
    /// Raw packet payload bytes
    pub data: Vec<u8>,
}

/// A complete sequence of a recording
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedSession {
    /// All packets in chronological order
    pub packets: Vec<RecordedPacket>,
}
