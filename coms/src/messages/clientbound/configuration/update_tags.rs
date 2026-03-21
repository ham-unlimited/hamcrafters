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
    /// A list of tagged registries, each containing a list of tags and their values.
    pub tagged_registries: PrefixedArray<TaggedRegistry>,
}

/// New tags for a specific registry.
#[derive(Debug, Deserialize)]
pub struct TaggedRegistry {
    /// The name of the registry.
    pub registry: String,
    /// A list of tags within the registry.
    pub tags: PrefixedArray<Tag>,
}

/// A list of block ids belonging to the specified tag (e.g. minecraft:climbable).
#[derive(Debug, Deserialize)]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// A list of numeric ids belonging to the tag.
    pub values: PrefixedArray<VarInt>,
}
