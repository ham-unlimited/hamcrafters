use serde::{Deserialize, Serialize};

use crate::McPacket;
use crate::codec::{identifier::Identifier, prefixed_array::PrefixedArray};
use mc_packet_macros::mc_packet;

/// A feature flags packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x0C)]
pub struct FeatureFlags {
    feature_flags: PrefixedArray<Identifier>,
}
