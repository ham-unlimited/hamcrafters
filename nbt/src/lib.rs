#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Crate for handling the NBT (Named Binary Tag) format used for different things in Minecraft
//! Supports parsing / serializing / deserializing NBT files.

use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use flate2::read::GzDecoder;

use crate::{error::NbtResult, nbt_named_tag::NbtNamedTag};

/// Error types for this crate.
pub mod error;
/// NBT Named Tag implementation.
pub mod nbt_named_tag;
/// NBT type implementations.
pub mod nbt_types;
/// Deserialize impl for nbt format.
pub mod nbt_value;
/// Serde implementations for NBT.
pub mod ser;
/// SNBT (Serialized Named Binary Tag) implementation.
pub mod snbt;
/// Tag type, wrapper for all NBT types.
pub mod tag_type;

/// Read & parse a gzipped NBT file from the provided path.
pub fn read_nbt_file(path: &Path) -> NbtResult<Option<NbtNamedTag>> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);

    let mut file_content = Vec::new();
    decoder.read_to_end(&mut file_content)?;

    let mut file_content = Cursor::new(file_content);

    let nbt = NbtNamedTag::read(&mut file_content)?;

    Ok(nbt)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::{Deserialize, Serialize};

    use crate::{
        nbt_types::{NbtCompound, NbtString},
        ser::serializer::to_nbt_tag_type,
        snbt::Snbt,
        tag_type::NbtTagType,
    };

    use super::*;

    #[test]
    fn test_parse_level_dat() {
        read_nbt_file(Path::new("../test-data/test-world/level.dat"))
            .expect("Expect to read file correctly");
    }

    #[test]
    fn test_maps() {
        let input = NbtTagType::TagCompound(NbtCompound(vec![NbtNamedTag {
            name: NbtString("my_map".to_string()),
            payload: NbtTagType::TagCompound(NbtCompound(vec![
                NbtNamedTag {
                    name: NbtString("first_value".to_string()),
                    payload: NbtTagType::TagString(NbtString("first_value_value".to_string())),
                },
                NbtNamedTag {
                    name: NbtString("second_value".to_string()),
                    payload: NbtTagType::TagString(NbtString("second_value_value".to_string())),
                },
            ])),
        }]));

        let deserializer = ser::deserializer::Deserializer::from_nbt_tag(input.clone());
        let pepe = Pepe::deserialize(deserializer).expect("Failed to deserialize map");
        let serialized = to_nbt_tag_type(&pepe).expect("Failed to serialize map");

        let input: Snbt = (&input).into();
        let serialized: Snbt = (&serialized.unwrap()).into();

        assert_eq!(serialized.to_string(), input.to_string());
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Pepe {
        my_map: BTreeMap<String, String>,
    }
}
