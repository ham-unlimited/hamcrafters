#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Crate for handling communication of packets between a server and client in Minecraft.

use std::io::{Read, Write};

use crate::{
    messages::McPacket,
    ser::{ReadingError, WritingError, deserializer, serializer},
};
use serde::{Serialize, de::DeserializeOwned};

/// Special special minecraft types e.g. VarInt etc.
pub mod codec;
/// Minecraft packet definitions.
pub mod messages;
/// Implements support for reading mc_packets correctly.
pub mod packet_reader;
/// Implements support for writing mc_packets correctly.
pub mod packet_writer;
/// Network coms for sending / receiving MC Packets.
pub mod ser;
/// Reading / writing (a bit unclear tbh).
pub mod serial;

/// A client-bound packet.
pub trait ClientPacket: McPacket {
    /// Write the data of client-bound packet to the provided [write].
    fn write_packet_data(&self, write: impl Write) -> Result<(), WritingError>;
}

/// A server-bound packet.
pub trait ServerPacket: McPacket + Sized {
    /// Read a server-bound packet from the provided [read].
    fn read(read: impl Read) -> Result<Self, ReadingError>;
}

impl<P: McPacket + Serialize> ClientPacket for P {
    fn write_packet_data(&self, write: impl Write) -> Result<(), WritingError> {
        let mut serializer = serializer::Serializer::new(write);
        self.serialize(&mut serializer)
    }
}

impl<P: McPacket + DeserializeOwned> ServerPacket for P {
    fn read(read: impl Read) -> Result<P, ReadingError> {
        let mut deserializer = deserializer::Deserializer::new(read);
        P::deserialize(&mut deserializer)
    }
}
