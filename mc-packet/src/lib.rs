#[forbid(unsafe_code)]
#[deny(missing_docs)]

/// Crate for keeping a McPacket trait (maybe more?)

/// A Minecraft packet
pub trait McPacket {
    /// The packet ID for this packet.
    const PACKET_ID: usize;
}
