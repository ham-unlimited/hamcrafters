use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Clientbound keep alive request.
#[derive(Debug, Deserialize)]
#[mc_packet(0x04)]
pub struct ClientboundKeepAlive {
    keep_alive_id: i64,
}
