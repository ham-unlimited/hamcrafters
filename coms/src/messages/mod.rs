use std::string::FromUtf8Error;

use serde::Deserialize;

use crate::{codec::var_int::VarInt, packet_reader::RawPacket, ser::ReadingError};

// TODO: remove when implemented all parsers

/// Client-bound packages.
#[allow(unused)]
pub mod clientbound;
/// Server-bound packages.
#[allow(unused)]
pub mod serverbound;

/// A Minecraft packet
pub trait McPacket: McPacketRead {
    /// The packet ID for this packet.
    const PACKET_ID: i32;

    /// Get the packet ID of this McPacket.
    fn get_packet_id() -> VarInt {
        VarInt(Self::PACKET_ID)
    }
}

/// Errors that can occurr during McPacket operations.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum McPacketError {
    #[error("Failed to read this packet, err: {0}")]
    ReadingError(#[from] ReadingError),
    #[error("IO error, err: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse utf8, err: `{0}`")]
    Utf8Error(#[from] FromUtf8Error),
}

/// Something that can read a Minecraft packet.
pub trait McPacketRead {
    /// Output when reading a packet from this type, (usually the type itself).
    type Output;

    /// Read a packet from the provided raw_packet.
    fn read(raw_packet: RawPacket) -> Result<Self::Output, McPacketError>;
}

impl<'de, T> McPacketRead for T
where
    T: Deserialize<'de>,
{
    type Output = T;

    fn read(raw_packet: RawPacket) -> Result<Self::Output, McPacketError> {
        let v = T::deserialize(&mut raw_packet.get_deserializer())?;

        Ok(v)
    }
}
