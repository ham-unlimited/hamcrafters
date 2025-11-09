use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Signals that configuration is finished.
#[derive(Debug, Serialize, Deserialize)]
#[mc_packet(0x03)]
pub struct FinishConfiguration;
