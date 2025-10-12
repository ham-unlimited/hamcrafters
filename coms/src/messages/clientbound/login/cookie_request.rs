use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[mc_packet(0x05)]
pub struct CookieRequest {
    key: String,
}
