use crate::{codec::var_int::VarInt, McPacket};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Sent by the client to confirm a teleportation initiated by the server.
///
/// The client must respond with the same teleport ID that was sent in the
/// [Synchronize Player Position](https://minecraft.wiki/w/Java_Edition_protocol/Packets#Synchronize_Player_Position)
/// packet. The server will not process movement packets from the client until
/// a matching confirmation is received.
#[derive(Debug, Deserialize)]
#[mc_packet(0x00)]
pub struct ConfirmTeleportation {
    /// The teleport ID from the corresponding Synchronize Player Position packet.
    pub teleport_id: VarInt,
}
