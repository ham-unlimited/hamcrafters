#![forbid(unsafe_code)]

//! Generation of Rust types for data files outputed by Minecraft data files.

pub mod packets;

use std::fs;

use eyre::Context;

use crate::packets::Packets;

const INPUT_PATH: &'static str = "../generated";
const GENERATED_DIR: &'static str = "../mc-codegen/src/generated";

pub fn main() -> eyre::Result<()> {
    fs::create_dir_all(GENERATED_DIR)
        .wrap_err("Failed to create generated code path directories")?;

    let packets = Packets::read_from_file(INPUT_PATH).wrap_err("Failed to read packets")?;
    println!("{packets:?}");

    Ok(())
}
