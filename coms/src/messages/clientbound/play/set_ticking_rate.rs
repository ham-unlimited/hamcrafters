use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Clientbound server data packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x7d)]
pub struct SetTickingRate {
    /// Tick rate in ticks per second.
    pub tick_rate: f32,
    /// Whether the ticking is frozen or not.
    pub is_frozen: bool,
}
