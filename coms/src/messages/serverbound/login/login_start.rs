use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[mc_packet(0x0)]
pub struct LoginStart {
    pub name: String,
    pub player_uuid: Uuid,
}
