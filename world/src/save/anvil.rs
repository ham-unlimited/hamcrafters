use core::time;
use std::{
    fs,
    io::{self, Cursor, Read},
    path::Path,
};

use flate2::read::ZlibDecoder;
use log::{error, info};
use nbt::{
    error::NbtError,
    nbt_types::{NbtString, NbtType},
};

use crate::save::chunk_data::{self, ChunkData};

type U24 = u32;

const CHUNKS_PER_REGION: usize = 1024;
const LOCATIONS_LENGTH: usize = CHUNKS_PER_REGION * size_of::<ChunkMetadata>();
const TIMESTAMPS_LENGTH: usize = CHUNKS_PER_REGION * size_of::<u32>();
const ANVIL_HEADER_LENGTH: usize = LOCATIONS_LENGTH + TIMESTAMPS_LENGTH;
// Number of chunks per sector side.
const SECTOR_CHUNK_WIDTH: usize = 16;
const SECTOR_SIZE: usize = CHUNKS_PER_REGION * 4;

#[derive(thiserror::Error, Debug)]
pub enum AnvilError {
    #[error("IO Error `{0}`")]
    IoError(#[from] io::Error),
    #[error("Data uses an unsupported compression algorithm \"{0}\"")]
    UnsupportedCompressionAlgorithm(String),
    #[error("Invalid compression_type value {0}")]
    InvalidCompressionType(u8),
    #[error("Failed parsing NBT {0}")]
    NbtError(#[from] NbtError),
    #[error("Chunk contained unexpected data")]
    InvalidChunkFormat,
    #[error("Failed to serialize/deserialize NBT chunk data, err: {0}")]
    NbtSerdeError(#[from] nbt::ser::Error),
}

pub type AnvilResult<T> = Result<T, AnvilError>;

#[derive(Debug, Clone)]
struct Region {
    data: Vec<u8>,
}

impl Region {
    fn from_file<P: AsRef<Path>>(path: P) -> AnvilResult<Self> {
        Ok(Self {
            data: fs::read(path)?,
        })
    }

    fn read_chunks(&self) -> AnvilResult<()> {
        for z in 0..32 {
            for x in 0..32 {
                let chunk = self.read_chunk_at(x, z)?;
                info!("Chunk: {chunk:?}");
            }
        }

        Ok(())
    }

    // TODO: Avoid potential panics when reading data (i.e. not directly indexing into it...).
    fn read_chunk_at(&self, x: usize, z: usize) -> AnvilResult<Option<ChunkMetadata>> {
        let loc_index = 4 * (x % 32) + ((z % 32) * 32);
        let ts_index = loc_index + SECTOR_SIZE;

        let offset = i32::from_be_bytes([
            0,
            self.data[loc_index],
            self.data[loc_index + 1],
            self.data[loc_index + 2],
        ]) as usize;
        let sector_count = self.data[loc_index + 3];

        if offset == 0 && sector_count == 0 {
            return Ok(None);
        }

        let timestamp = u32::from_be_bytes([
            self.data[ts_index],
            self.data[ts_index + 1],
            self.data[ts_index + 2],
            self.data[ts_index + 3],
        ]);

        info!(
            "Chunk at {x} {z} has offset {offset}, was last modified at {timestamp} and contains {sector_count} sectors"
        );
        for sector in 0..sector_count {
            let sector_start = offset * SECTOR_SIZE;
            let sector_data_length = i32::from_be_bytes([
                self.data[sector_start],
                self.data[sector_start + 1],
                self.data[sector_start + 2],
                self.data[sector_start + 3],
            ]) - 1; // -1 for compression_type byte.

            let sector_data_start = sector_start + 5; // 4 for length and 1 byte for compression type.
            let sector_data_end = sector_data_start + sector_data_length as usize;
            let mut sector_reader = Cursor::new(&self.data[sector_data_start..sector_data_end]);

            info!("\tSector {sector} has length {sector_data_length}");

            let mut chunk_buf = Vec::new();
            match self.data[sector_start + 4] {
                1 => {
                    info!("\t\tGzip compressed");
                }
                2 => {
                    info!("\t\tZlip compressed");
                    ZlibDecoder::new(sector_reader).read_to_end(&mut chunk_buf)?;
                }
                3 => {
                    info!("\t\tUncompressed");
                }
                4 => {
                    info!("\t\tLZ4 Compressed");
                }
                127 => {
                    info!("\t\tCustom compression algorithm");
                    let s = NbtString::read(&mut sector_reader)?;
                    return Err(AnvilError::UnsupportedCompressionAlgorithm(s.0));
                }
                c => {
                    error!("\t\tInvalid compression_type {c}");
                    return Err(AnvilError::InvalidCompressionType(c));
                }
            }

            let chunk_data = ChunkData::read(&mut Cursor::new(chunk_buf))?;
        }

        // TODO
        Ok(None)
    }
}

#[derive(Debug, Clone)]
struct ChunkMetadata {
    offset: usize,
    sector_count: u8,
    timestamp: u32,
    sectors: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use crate::save::anvil::Region;

    #[test]
    fn parse_file_successfully() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }

        pretty_env_logger::init();

        // let file = std::fs::File::open("../test-data/test-world/region/r.0.0.mca").unwrap();
        // let mut fa = fastanvil::Region::from_stream(file).unwrap();
        // let chunk = fa.read_chunk(0, 0).unwrap();
        // log::info!("Chunk {}", chunk.unwrap().iter().len());

        let region = Region::from_file("../test-data/test-world/region/r.0.0.mca")
            .expect("Failed to open region");
        region
            .read_chunks()
            .expect("Failed to read chunks in region");

        assert_eq!(true, false);
    }
}
