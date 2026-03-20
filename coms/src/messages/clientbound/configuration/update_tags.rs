use mc_packet_macros::mc_packet;
use serde::Deserialize;

use crate::{
    McPacket,
    codec::{prefixed_array::PrefixedArray, var_int::VarInt},
};

/// Update the tags registry of the client.
#[derive(Debug, Deserialize)]
#[mc_packet(0x0D)]
pub struct UpdateTags {
    tagged_registries: PrefixedArray<TaggedRegistry>,
}

/// New tags for a specific registry.
#[derive(Debug, Deserialize)]
pub struct TaggedRegistry {
    registry: String,
    tags: PrefixedArray<Tag>,
}

/// A list of block ids belonging to the specified tag (e.g. minecraft:climbable).
#[derive(Debug, Deserialize)]
pub struct Tag {
    name: String,
    values: PrefixedArray<VarInt>,
}
