use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray},
};
use mc_packet_macros::mc_packet;
use nbt::{nbt_named_tag::NbtNamedTag, nbt_value::value::NbtValue};
use serde::Deserialize;

/// Registry data packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x07)]
pub struct RegistryData {
    registry_id: Identifier,
    entries: PrefixedArray<RegistryEntry>,
}

/// A specific registry data entry.
#[derive(Debug, Deserialize)]
pub struct RegistryEntry {
    entry_id: Identifier,
    data: Option<NbtValue>,
}
