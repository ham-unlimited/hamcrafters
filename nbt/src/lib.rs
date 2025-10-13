use std::{
    fs::File,
    io::{self, Cursor, Read},
    num::TryFromIntError,
    path::Path,
    string::FromUtf8Error,
};

use ::serde::Deserialize;
use flate2::read::GzDecoder;

use crate::nbt_named_tag::NbtNamedTag;

pub mod error;
pub mod nbt_named_tag;
pub mod nbt_types;
pub mod serde;
pub mod tag_type;

#[derive(Debug, thiserror::Error)]
pub enum NbtError {
    #[error("IO Error `{0}`")]
    IoError(#[from] io::Error),
    #[error("Encountered invalid nbt tag {0}")]
    InvalidNbtTag(u8),
    #[error("Malformed NBT, err: {0}")]
    MalformedNbt(String),
    #[error("String contained invalid utf8, err: {0}")]
    InvalidUtf8(#[from] FromUtf8Error),
    #[error("Provided length was invalid, err: {0}")]
    InvalidLength(#[from] TryFromIntError),
}

pub type NbtResult<T> = Result<T, NbtError>;

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
    use crate::serde::deserializer::Deserializer;

    use super::*;

    #[test]
    fn test_parse_level_dat() {
        read_nbt_file(Path::new("../test-data/level.dat")).expect("Expect to read file correctly");
    }

    #[test]
    fn test_deserialize_level_dat() {
        let Some(nbt) = read_nbt_file(Path::new("../test-data/level.dat"))
            .expect("Expect to read file correctly")
        else {
            panic!("Failed");
        };

        let deserializer = Deserializer::from_nbt_tag(nbt.payload);
        let dat = MinecraftLevelDat::deserialize(deserializer).expect("Faield to deserialize");
    }
}

#[derive(Deserialize)]
struct MinecraftLevelDat {
    #[serde(rename = "Data")]
    data: MinecraftLevelDatData,
}

#[derive(Deserialize)]
struct MinecraftLevelDatData {
    #[serde(rename = "allowCommands")]
    allow_commands: bool,
}
