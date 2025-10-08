use std::io::Write;

use thiserror::Error;

use crate::{codec::var_int::VarInt, messages::McPacket, ser::NetworkWriteExt};

#[derive(Error, Debug)]
pub enum PacketWriteError {}

pub struct NetworkWriter<W: Write> {
    writer: W,
}

impl<W: Write> NetworkWriter<W> {
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
    pub fn write_packet<P: McPacket>(&mut self, packet: P) -> Result<(), PacketWriteError> {
        let mut packet_buffer = Vec::new();
        packet_buffer.write_var_int(&VarInt(P::PACKET_ID as i32));

        todo!()
    }
}
