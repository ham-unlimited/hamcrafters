use crate::{
    McPacket,
    codec::{prefixed_array::PrefixedArray, var_int::VarInt},
};
use mc_packet_macros::mc_packet;
use nbt::tag_type::NbtTagType;
use serde::Deserialize;

/// Clientbound level chunk with light packet during play phase.
/// Sent by the server when a chunk comes into the client's view distance.
/// Chunk sections are sent in this packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x2C)]
pub struct LevelChunkWithLight {
    /// The X coordinate of the chunk.
    pub chunk_x: i32,
    /// The Z coordinate of the chunk.
    pub chunk_z: i32,
    /// Array of heightmaps for this chunk. Each heightmap encodes the highest occupied block per column.
    pub heightmaps: PrefixedArray<Heightmap>,
    /// The raw chunk section data encoded as a byte array (VarInt-prefixed length).
    pub data: PrefixedArray<u8>,
    /// The block entities present in this chunk.
    pub block_entities: PrefixedArray<ChunkBlockEntity>,
    /// BitSet containing which sections relative to the world's minimum Y have their sky light populated.
    /// Each bit represents one chunk section (ordered bottom to top). The array has ceil((max_sections + 2) / 64) longs.
    pub sky_light_mask: PrefixedArray<i64>,
    /// BitSet containing which sections relative to the world's minimum Y have their block light populated.
    pub block_light_mask: PrefixedArray<i64>,
    /// BitSet containing which sections relative to the world's minimum Y are empty of sky light.
    pub empty_sky_light_mask: PrefixedArray<i64>,
    /// BitSet containing which sections relative to the world's minimum Y are empty of block light.
    pub empty_block_light_mask: PrefixedArray<i64>,
    /// The sky light data arrays, one per section set in sky_light_mask. Each inner array is exactly 2048 bytes.
    pub sky_light_arrays: PrefixedArray<PrefixedArray<u8>>,
    /// The block light data arrays, one per section set in block_light_mask. Each inner array is exactly 2048 bytes.
    pub block_light_arrays: PrefixedArray<PrefixedArray<u8>>,
}

/// A single heightmap entry for a chunk.
/// Encodes the position of the highest occupied block in each of the 256 block columns.
#[derive(Debug, Deserialize)]
pub struct Heightmap {
    /// The type of heightmap (e.g. WORLD_SURFACE = 1, MOTION_BLOCKING = 4).
    pub heightmap_type: VarInt,
    /// Packed long array encoding height values (ceil(log2(world_height + 1)) bits per entry, 256 entries total).
    pub data: PrefixedArray<i64>,
}

/// A block entity within a chunk.
#[derive(Debug, Deserialize)]
pub struct ChunkBlockEntity {
    /// The packed XZ position within the chunk: (blockEntityX & 15) << 4 | (blockEntityZ & 15).
    pub packed_xz: u8,
    /// The Y coordinate of the block entity.
    pub y: i16,
    /// The block entity type ID (index into minecraft:block_entity_type registry).
    pub entity_type: VarInt,
    /// The NBT data for this block entity, without the X, Y, and Z values.
    pub data: NbtTagType,
}
