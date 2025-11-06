use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Login acknowledged msg
#[derive(Debug, Deserialize, Serialize)]
#[mc_packet(0x03)]
pub struct LoginAcknowledged;
