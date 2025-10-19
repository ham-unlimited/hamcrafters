use std::io::Read;

use log::info;
use nbt::{nbt_named_tag::NbtNamedTag, ser::deserializer::Deserializer};
use serde::Deserialize;

use crate::save::anvil::{AnvilError, AnvilResult};

// Information taken from here: https://minecraft.fandom.com/wiki/Chunk_format
// Although we should probably also consider this page: https://minecraft.fandom.com/wiki/Anvil_file_format
#[derive(Deserialize, Debug)]
pub struct ChunkData {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    #[serde(rename = "xPos")]
    x_pos: i32,
    #[serde(rename = "yPos")]
    y_pos: i32,
    #[serde(rename = "zPos")]
    z_pos: i32,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "LastUpdate")]
    last_update: i64,
    // TODO: sections
    // TODO: block_entities
    // TODO: CarvingMasks? (Maybe not a thing anymore).
    // TODO: Heightmaps
    // TODO: Lights
    // TODO: Entities
    // TODO: fluid_ticks
    // TODO: block_ticks
    #[serde(rename = "InhabitedTime")]
    inhabited_time: i64,
    // TODO: PostProcessing
    // TODO: structures
}

impl ChunkData {
    pub fn read<R: Read>(reader: &mut R) -> AnvilResult<Self> {
        let Some(tag) = NbtNamedTag::read(reader)? else {
            return Err(AnvilError::InvalidChunkFormat);
        };

        let deserializer = Deserializer::from_nbt_tag(tag.payload);
        let cd = ChunkData::deserialize(deserializer)?;
        info!("Chunk data: {cd:?}");
        todo!()
    }
}
