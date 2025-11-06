use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// A Minecraft ping request packet.
#[derive(Debug, Deserialize, Serialize)]
#[mc_packet(0x01)]
pub struct PingRequest {
    /// The time in milliseconds since the Minecraft client was started.
    pub timestamp: i64,
}
