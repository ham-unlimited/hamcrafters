use std::io::{self, Cursor};

use crate::{
    codec::var_int::VarInt,
    ser::{ReadingError, deserializer::Deserializer},
};
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt};

/// Error occurred during the reading of a packet.
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum PacketReadError {
    #[error("IO Error occurred during writing `{0}`")]
    IoError(#[from] io::Error),
    #[error("Connection was closed")]
    ConnectionClosed,
    #[error("Failed to parse length `{0}`")]
    LengthParseError(String),
    #[error("The received packet ID was not valid")]
    InvalidPacketId,
    #[error("Failed to read packet data `{0}`")]
    PacketDataReadError(String),
}

/// Reader for reading packets from the network based on the underlying [reader].
pub struct NetworkReader<R: AsyncRead + Unpin> {
    reader: R,
}

/// A generic minecraft packet that has yet to be parsed into its specific packet type.
#[derive(Debug, Clone)]
pub struct RawPacket {
    /// The ID of the packet, should be unique per context (server/client-bound) / state.
    pub id: i32,
    /// The payload of the packet.
    pub data: Vec<u8>,
}

impl RawPacket {
    /// Convert this into a deserializer, we're not implementing the From trait to avoid name confusions with other deserializers.
    pub fn get_deserializer(self) -> Deserializer<Cursor<Vec<u8>>> {
        let cursor = Cursor::new(self.data);
        Deserializer::new(cursor)
    }
}

impl<R: AsyncRead + Unpin> NetworkReader<R> {
    /// Create a new [NetworkReader] utilizing the provided [reader] as a basis for incoming packets.
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    /// Read a single [RawPacket] from the [reader].
    pub async fn get_packet(&mut self) -> Result<RawPacket, PacketReadError> {
        // TODO: handle connection closed?

        let packet_len = VarInt::decode_async(&mut self.reader)
            .await
            .map_err(|err| match err {
                ReadingError::CleanEOF(_) => PacketReadError::ConnectionClosed,
                err => PacketReadError::LengthParseError(err.to_string()),
            })?;

        let packet_len = packet_len.0 as u64;

        // TODO: Validate packet length.

        let mut packet_reader = (&mut self.reader).take(packet_len);

        let packet_id = VarInt::decode_async(&mut packet_reader)
            .await
            .map_err(|_| PacketReadError::InvalidPacketId)?;

        let mut packet_data = Vec::new();
        packet_reader
            .read_to_end(&mut packet_data)
            .await
            .map_err(|err| PacketReadError::PacketDataReadError(err.to_string()))?;

        Ok(RawPacket {
            id: packet_id.0,
            data: packet_data,
        })
    }
}
