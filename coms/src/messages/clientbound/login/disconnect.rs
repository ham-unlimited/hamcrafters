use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Disconnect message, gives a reason as a json object
#[derive(Debug, Deserialize)]
#[mc_packet(0x00)]
pub struct Disconnect {
    reason: String, // json
}
