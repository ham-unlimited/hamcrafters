pub mod clientbound;
pub mod serverbound;

/// A Minecraft packet
pub trait McPacket {
    /// The packet ID for this packet.
    const PACKET_ID: i32;
}
