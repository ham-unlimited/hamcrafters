use crate::codec::var_int::VarInt;

// TODO: remove when implemented all parsers

/// Client-bound packages.
#[allow(unused)]
pub mod clientbound;
/// Server-bound packages.
#[allow(unused)]
pub mod serverbound;

/// A Minecraft packet
pub trait McPacket {
    /// The packet ID for this packet.
    const PACKET_ID: i32;

    /// Get the packet ID of this McPacket.
    fn get_packet_id() -> VarInt {
        VarInt(Self::PACKET_ID)
    }
}
