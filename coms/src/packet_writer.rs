use std::io;

use log::error;
use serde::Serialize;
use thiserror::Error;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::{
    ClientPacket,
    codec::var_int::VarInt,
    messages::McPacket,
    ser::{NetworkWriteExt, WritingError},
};

/// Error that occurrs during the writing of a packet.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum PacketWriteError {
    #[error("Writing error `{0}`")]
    WritingError(#[from] WritingError),
    #[error("Packet length was too long to fit into VarInt")]
    PacketLengthTooLarge,
    #[error("IO Error occurred during writing `{0}`")]
    IoError(#[from] io::Error),
}

/// A writing for writing packets to the network.
pub struct NetworkWriter<W: AsyncWrite + Unpin> {
    writer: W,
}

impl<W: AsyncWrite + Unpin> NetworkWriter<W> {
    /// Returns a new [NetworkWriter] using the provided [writer] as output.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Writes a mc_packet to the internal writer.
    ///
    /// Packet structure:
    ///
    /// Packet Length (VarInt)
    /// Packet ID (VarInt)
    /// Data (Optional, Bytes)
    pub async fn write_packet<P: McPacket + Serialize>(
        &mut self,
        packet: P,
    ) -> Result<(), PacketWriteError> {
        let mut packet_buffer = Vec::new();
        packet_buffer.write_var_int(&P::get_packet_id())?;
        packet.write_packet_data(&mut packet_buffer)?;

        let packet_length: VarInt = packet_buffer.len().try_into().map_err(|err| {
            error!("Packet length was too large to fit into VarInt! (err: {err:?})");
            PacketWriteError::PacketLengthTooLarge
        })?;

        packet_length.encode_async(&mut self.writer).await?;
        self.writer.write_all(&packet_buffer).await?;

        self.writer.flush().await?;

        Ok(())
    }
}
