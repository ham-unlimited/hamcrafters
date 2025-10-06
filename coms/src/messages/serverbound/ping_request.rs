use serde::Deserialize;

use crate::messages::McPacket;

#[derive(Debug, Deserialize)]
pub struct PingRequest {
    pub timestamp: i64,
}

impl McPacket for PingRequest {
    fn packet_id() -> &'static usize {
        &0x01
    }
}
