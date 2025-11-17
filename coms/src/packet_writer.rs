use std::io::{self, Cursor};

#[allow(deprecated)]
use aes::cipher::{BlockEncryptMut, BlockSizeUser, KeyIvInit, generic_array::GenericArray};
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
    encryption_key: Option<Encryption>,
}

impl<W: AsyncWrite + Unpin> NetworkWriter<W> {
    /// Returns a new [NetworkWriter] using the provided [writer] as output and no encryption.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            encryption_key: None,
        }
    }

    /// Enable encryption for this writer. Note: once enabled, you cannot disable encryption.
    pub fn enable_encryption(&mut self, key: &[u8; 16]) -> Result<(), PacketWriteError> {
        // TODO: Check that encryption isn't already enabled.

        log::info!("Enabling encryption for writer");
        let cipher =
            cfb8::Encryptor::<aes::Aes128>::new_from_slices(key, key).expect("invalid key");

        self.encryption_key = Some(Encryption { cipher });

        Ok(())
    }

    /// Write some data using this writer.
    pub async fn write_data(&mut self, data: Vec<u8>) -> Result<(), PacketWriteError> {
        match self.encryption_key.as_mut() {
            Some(s) => {
                let mut bf = Vec::new();

                // Encrypt the raw data, note that our block size is 1 byte, so this is always safe
                for block in data.chunks(cfb8::Encryptor::<aes::Aes128>::block_size()) {
                    let mut out = [0u8];

                    // This is a stream cipher, so this value must be used
                    // TODO: Wait for aes/crypto-common to update generic array
                    #[expect(deprecated)]
                    let out_block = GenericArray::from_mut_slice(&mut out);
                    s.cipher.encrypt_block_b2b_mut(block.into(), out_block);

                    bf.push(out_block[0]);
                }

                self.writer.write_all(&bf).await?;
            }
            None => {
                self.writer.write_all(&data).await?;
            }
        }

        self.writer.flush().await?;

        Ok(())
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

        let mut buffer = Vec::new();
        packet_length.encode(&mut buffer)?;
        io::copy(&mut Cursor::new(packet_buffer), &mut buffer)?;

        self.write_data(buffer).await
    }
}

struct Encryption {
    cipher: cfb8::Encryptor<aes::Aes128>,
}
