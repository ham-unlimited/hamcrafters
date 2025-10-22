use std::io::Read;

use log::warn;
use nbt::{nbt_named_tag::NbtNamedTag, ser::deserializer::Deserializer};
use serde::{Deserialize, de::Visitor};

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
    sections: Vec<Section>,
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

        // TODO: Finish parsing the data.

        Ok(cd)
    }
}

/// A section in Minecraft (also known as a sub-chunk), it covers the same 16x16 area but only 16 blocks tall so a total of 4096 blocks.
/// This means that there are (currently) 24 subchunks per chunk in the overworld of Minecraft.
#[derive(Deserialize, Debug)]
pub struct Section {
    #[serde(rename = "Y")]
    y: i8,
    block_states: BlockStates,
    biomes: Biomes, // TODO
    #[serde(rename = "BlockLight")]
    block_light: Option<Vec<i8>>, // Always 2048 long but serde only supports arrays of length 0..=32 (each half-byte is one block). Omitted if there is no light that reaches this section.
    #[serde(rename = "SkyLight")]
    sky_light: Option<Vec<i8>>, // Always 2048 long but serde only supports arrays of length 0..=32 (each half-byte is one block). If omitted we should look at the section right above it.
}

#[derive(Deserialize, Debug)]
pub struct BlockStates {
    palette: Vec<Block>, // Up to 4096 long in vanilla, longer are supported for other servers / clients.
    data: Option<Vec<i64>>, // Always 4096 long, points to indices in the palette vector for each block of the section. Omitted if a single blockstate is used for the entire section.
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Block {
    name: String, // Block resource location.
    properties: Option<PropertiesList>,
}

#[derive(Debug)]
struct PropertiesList(Vec<BlockState>);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BlockState {
    Age(SI32),
    Axis(BlockAxis),
    Down(SBool),
    Level(SI32),
    Up(SBool),
    West(SBool),
    North(SBool),
    South(SBool),
    East(SBool),
    Lit(SBool),
    Waterlogged(SBool),
    Half(Half),
    Persistent(SBool),
    Distance(SI32),
    SegmentAmount(SI32),
    Facing(Direction),
    Snowy(SBool),
    Drag(SBool),
    Type(BlockType),
    Shape(BlockShape),
    Open(SBool),
    Powered(SBool),
    Unsupported { name: String, value: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlockType {
    Normal,
    Sticky,
    Left,
    Right,
    Single,
    Bottom,
    Double,
    Top,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockShape {
    /* Rail */
    AscendingEast,
    AscendingNorth,
    AscendingSouth,
    AscendingWest,
    EastWest,
    NorthSouth,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    /* Stairs */
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
    Straight,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Down,
    East,
    North,
    South,
    West,
    Up,
}

/// A stringified boolean, because sometimes minecraft decides to put booleans in Strings :facepalm:
#[derive(Debug)]
pub struct SBool(bool);

impl<'de> Deserialize<'de> for SBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "true" => Ok(SBool(true)),
            "false" => Ok(SBool(false)),
            o => Err(serde::de::Error::custom(format!(
                "Invalid value for SBool '{o}'"
            ))),
        }
    }
}

/// A stringified i32, because sometimes minecraft decides to put integers in Strings :facepalm:
#[derive(Debug)]
pub struct SI32(i32);

impl<'de> Deserialize<'de> for SI32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let i = s.parse::<i32>().map_err(|err| {
            serde::de::Error::custom(format!(
                "Failed to parse SI32 from value '{s}' due to err: {err}"
            ))
        })?;

        Ok(SI32(i))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BlockAxis {
    X,
    Y,
    Z,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Half {
    Upper,
    Lower,
    Bottom,
    Top,
}

struct PropertiesVisitor;

impl<'de> Visitor<'de> for PropertiesVisitor {
    type Value = PropertiesList;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Expected block state object")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut properties = vec![];

        // CRIMES AHOY! But hey, how would you solve it?
        while let Some((field, value)) = map.next_entry::<String, serde_json::Value>()? {
            let f = field.clone();
            let v = value.clone();
            let json_obj = serde_json::json!({ f: v });
            let block_state: BlockState = match serde_json::from_value(json_obj) {
                Ok(b) => b,
                Err(err) => {
                    warn!(
                        "Failed to deserialize property: {{{field}: {value}}} converting to 'Unsupported', err: {err}"
                    );
                    BlockState::Unsupported {
                        name: field,
                        value: value.to_string(),
                    }
                }
            };
            properties.push(block_state);
        }

        Ok(PropertiesList(properties))
    }
}

impl<'de> Deserialize<'de> for PropertiesList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(PropertiesVisitor)
    }
}

#[derive(Deserialize, Debug)]
struct Biomes {
    palette: Vec<String>, // Will never contain more than 64 entries in vanilla but larger are supported.
    data: Option<Vec<i64>>, // Contains 64 indices pointing ot the palette, biomes are stored in cells of 4x4x4 blocks. Not provided if only one biome is used for this section.
}
