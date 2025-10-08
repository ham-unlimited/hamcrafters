use std::io::{Read, Write};

use crate::{
    messages::McPacket,
    ser::{ReadingError, WritingError, deserializer, serializer},
};
use serde::{Serialize, de::DeserializeOwned};

#[forbid(unsafe_code)]

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

pub trait ClientPacket: McPacket {
    fn write_packet_data(&self, write: impl Write) -> Result<(), WritingError>;
}

pub trait ServerPacket: McPacket + Sized {
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
