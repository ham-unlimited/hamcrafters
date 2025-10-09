/// Serde deserialization implementations for Minecraft types.
pub mod deserializer;
/// Serde serialization implementations for Minecraft types.
pub mod serializer;

use std::io::{Error, Read, Write};

/// A writable packet.
pub trait PacketWrite {
    /// Write the packet to the provided [writer].
    fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
    /// Write the packet to the provided [writer] using Big-Endian ordering.
    fn write_be<W: Write>(&self, _writer: &mut W) -> Result<(), Error> {
        panic!("not implemented")
    }
}

/// A readable packet.
pub trait PacketRead: Sized {
    /// Read a packet from the provided [reader].
    fn read<R: Read>(reader: &mut R) -> Result<Self, Error>;
    /// Read a packet from the provided [reader] using Big-Endian ordering.
    fn read_be<R: Read>(_reader: &mut R) -> Result<Self, Error> {
        panic!("not implemented")
    }
}
