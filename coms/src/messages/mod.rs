use crate::codec::var_int::VarInt;

pub mod clientbound;
pub mod serverbound;

/// A Minecraft packet
pub trait McPacket {
    /// The packet ID for this packet.
    const PACKET_ID: i32;

    fn get_packet_id() -> VarInt {
        VarInt(Self::PACKET_ID)
    }
}
