use mc_packet::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[mc_packet(0x00)]
pub struct Disconnect {
    pub reason: String, // json
}
