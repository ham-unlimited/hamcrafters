use serde::Deserialize;

use crate::{McPacket, codec::prefixed_array::PrefixedArray};
use mc_packet_macros::mc_packet;

/// Known packs requests.
#[derive(Debug, Deserialize)]
#[mc_packet(0x0E)]
pub struct ClientboundKnownPacks {
    known_packs: PrefixedArray<Pack>,
}

/// A datapack.
#[derive(Debug, Deserialize)]
pub struct Pack {
    namespace: String,
    id: String,
    version: String,
}
