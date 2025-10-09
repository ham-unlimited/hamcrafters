use std::io::{self, Cursor};

use crate::{
    codec::var_int::VarInt,
    ser::{
        ReadingError,
        deserializer::{self, Deserializer},
    },
};
use bytes::Bytes;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt};

#[derive(Error, Debug)]
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

pub struct NetworkReader<R: AsyncRead + Unpin> {
    reader: R,
}

#[derive(Debug, Clone)]
pub struct RawPacket {
    pub id: i32,
    pub data: Vec<u8>,
}

impl RawPacket {
    pub fn get_deserializer(self) -> Deserializer<Cursor<Vec<u8>>> {
        let cursor = Cursor::new(self.data);
        Deserializer::new(cursor)
    }
}

impl<R: AsyncRead + Unpin> NetworkReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

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
