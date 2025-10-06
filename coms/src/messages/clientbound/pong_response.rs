use serde::Deserialize;

use crate::messages::McPacket;
use crate::messages::serverbound::ping_request::PingRequest;

#[derive(Debug, Clone, Deserialize)]
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

impl McPacket for PongResponse {
    fn packet_id() -> &'static usize {
        &0x01
    }
}
