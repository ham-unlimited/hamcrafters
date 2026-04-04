use crate::{
    McPacket, codec::prefixed_array::PrefixedArray, messages::models::text_component::TextComponent,
};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Clientbound server data packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x54)]
pub struct ServerData {
    /// The MOTD of the server.
    pub motd: TextComponent,
    /// Icon bytes in the PNG format.
    pub icon: Option<PrefixedArray<i8>>,
}
