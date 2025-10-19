use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Login acknowledged msg
#[derive(Debug, Deserialize)]
#[mc_packet(0x03)]
pub struct LoginAcknowledged;
