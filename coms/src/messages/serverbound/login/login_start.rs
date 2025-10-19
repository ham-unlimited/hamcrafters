use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;
use uuid::Uuid;

/// Login start message
#[derive(Debug, Deserialize)]
#[mc_packet(0x0)]
pub struct LoginStart {
    name: String,
    player_uuid: Uuid,
}
