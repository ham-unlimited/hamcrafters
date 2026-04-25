use crate::{
    McPacket,
    codec::{prefixed_array::PrefixedArray, var_int::VarInt},
    ser::{NetworkReadExt, ReadingError},
};
use mc_packet_macros::mc_packet;
use nbt::tag_type::NbtTagType;
use serde::{
    Deserialize,
    de,
};
use std::io::{Cursor, Read};

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
    /// Chunk section data. The byte count is VarInt-prefixed; section count is not encoded — sections
    /// are parsed until the byte region is exhausted (section count depends on world height / dimension).
    pub data: ChunkSections,
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

/// All chunk sections for the chunk column.
/// Deserialized from a VarInt-prefixed raw byte array. The section count is not encoded in the packet;
/// sections are parsed until the byte region is exhausted (count depends on world height / dimension).
#[derive(Debug)]
pub struct ChunkSections(pub Vec<ChunkSection>);

impl<'de> Deserialize<'de> for ChunkSections {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = PrefixedArray::<u8>::deserialize(deserializer)?;
        let bytes = bytes.take_inner();
        let total = bytes.len() as u64;
        let mut cursor = Cursor::new(bytes);
        let mut sections = Vec::new();
        while cursor.position() < total {
            let section = ChunkSection::read(&mut cursor)
                .map_err(|e| de::Error::custom(format!("failed to parse chunk section: {e}")))?;
            sections.push(section);
        }
        Ok(ChunkSections(sections))
    }
}

/// One 16×16×16 chunk section (sub-chunk).
#[derive(Debug)]
pub struct ChunkSection {
    /// Number of non-air blocks in this section.
    pub block_count: i16,
    /// Block state paletted container (4096 = 16×16×16 entries).
    pub block_states: PalettedContainer,
    /// Biome paletted container (64 = 4×4×4 entries).
    pub biomes: PalettedContainer,
}

impl ChunkSection {
    fn read<R: Read>(r: &mut R) -> Result<Self, ReadingError> {
        let block_count = r.get_i16_be()?;
        let block_states = PalettedContainer::read(r, 4096, 8)?;
        let biomes = PalettedContainer::read(r, 64, 3)?;
        Ok(ChunkSection {
            block_count,
            block_states,
            biomes,
        })
    }
}

/// A paletted container for block states or biomes.
///
/// Three modes determined by `bits_per_entry`:
/// - **Single-valued** (BPE = 0): entire container uses one registry ID; no data array.
/// - **Indirect** (BPE ≤ max_indirect_bits for that container type): palette maps indices to IDs;
///   data array holds packed indices.
/// - **Direct** (BPE > max_indirect_bits): IDs stored directly in the data array; no palette.
///
/// The data array length is **calculated** (NOT sent since 1.21.5):
/// `ceil(entries / floor(64 / bits_per_entry))`, where entries = 4096 (block states) or 64 (biomes).
#[derive(Debug)]
pub struct PalettedContainer {
    /// Number of bits used per entry in the data array. 0 means single-valued mode.
    pub bits_per_entry: u8,
    /// The palette for this container, determining how data array entries map to registry IDs.
    pub palette: Palette,
    /// Packed longs comprising the data array. Empty for single-valued containers.
    pub data: Vec<i64>,
}

/// Palette mode for a [`PalettedContainer`].
#[derive(Debug)]
pub enum Palette {
    /// BPE = 0. The single registry ID that fills the entire container.
    SingleValued(VarInt),
    /// BPE in [1, max_indirect_bits]. List of registry IDs indexed by the data array entries.
    Indirect(Vec<VarInt>),
    /// BPE > max_indirect_bits. Data array entries are registry IDs directly.
    Direct,
}

impl PalettedContainer {
    fn read<R: Read>(r: &mut R, entries: u32, max_indirect_bits: u8) -> Result<Self, ReadingError> {
        let bpe = r.get_u8()?;

        let palette = if bpe == 0 {
            let value = VarInt::decode(r)?;
            Palette::SingleValued(value)
        } else if bpe <= max_indirect_bits {
            let palette_len = VarInt::decode(r)?;
            let mut palette_entries = Vec::with_capacity(palette_len.0 as usize);
            for _ in 0..palette_len.0 {
                palette_entries.push(VarInt::decode(r)?);
            }
            Palette::Indirect(palette_entries)
        } else {
            Palette::Direct
        };

        let data_long_count = if bpe == 0 {
            0
        } else {
            let values_per_long = 64u32 / bpe as u32;
            ((entries + values_per_long - 1) / values_per_long) as usize
        };

        let mut data = Vec::with_capacity(data_long_count);
        for _ in 0..data_long_count {
            data.push(r.get_i64_be()?);
        }

        Ok(PalettedContainer {
            bits_per_entry: bpe,
            palette,
            data,
        })
    }
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
