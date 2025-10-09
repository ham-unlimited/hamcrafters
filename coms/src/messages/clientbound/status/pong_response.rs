use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Serialize;

use crate::messages::serverbound::status::ping_request::PingRequest;

/// A Minecraft PongResponse.
#[derive(Debug, Clone, Serialize)]
#[mc_packet(0x01)]
pub struct PongResponse {
    /// The timestamp provided in the [PingRequest] that generated this response.
    pub timestamp_ms: i64,
}

impl From<PingRequest> for PongResponse {
    fn from(value: PingRequest) -> Self {
        Self {
            timestamp_ms: value.timestamp,
        }
    }
}
