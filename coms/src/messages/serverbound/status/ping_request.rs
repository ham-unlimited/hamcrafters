use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// A Minecraft ping request packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x01)]
pub struct PingRequest {
    /// The time in milliseconds since the Minecraft client was started.
    pub timestamp: i64,
}
