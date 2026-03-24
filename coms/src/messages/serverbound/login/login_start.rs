use crate::{McPacket, codec::mc_uuid::McUuid};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Login start message
#[derive(Debug, Deserialize)]
#[mc_packet(0x0)]
pub struct LoginStart {
    name: String,
    player_uuid: McUuid,
}
