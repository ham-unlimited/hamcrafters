use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

use crate::messages::serverbound::status::ping_request::PingRequest;

#[derive(Debug, Clone, Deserialize)]
#[mc_packet(0x01)]
pub struct PongResponse {
    pub timestamp_ms: i64,
}

impl From<PingRequest> for PongResponse {
    fn from(value: PingRequest) -> Self {
        Self {
            timestamp_ms: value.timestamp,
        }
    }
}
