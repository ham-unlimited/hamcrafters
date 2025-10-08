use std::io::{self, Write};

use log::error;
use serde::Serialize;
use thiserror::Error;
use tokio::io::AsyncWrite;

use crate::{
    ClientPacket,
    codec::var_int::VarInt,
    messages::McPacket,
    ser::{NetworkWriteExt, WritingError},
};

#[derive(Error, Debug)]
pub enum PacketWriteError {
    #[error("Writing error `{0}`")]
    WritingError(#[from] WritingError),
    #[error("Packet length was too long to fit into VarInt")]
    PacketLengthTooLarge,
    #[error("IO Error occurred during writing `{0}`")]
    IoError(#[from] io::Error),
}

pub struct NetworkWriter<W: AsyncWrite> {
    writer: W,
}

impl<W: AsyncWrite> NetworkWriter<W> {
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

        let mut len_buf = Vec::new();
        packet_length.encode(&mut len_buf)?;
        packet_length.encode(&mut self.writer).await?;
        self.writer.write_all(&packet_buffer)?;

        self.writer.flush()?;

        Ok(())
    }
}
