use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// A Minecraft serverbound plugin message packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x2)]
#[serde(rename_all = "PascalCase")]
pub struct ServerboundPluginMessage {
    /// Name of the plugin channel used to send this message.
    pub channel: String, // TODO: Really an identifier
                         // /// Data, depending on the channel.
                         // pub data: Vec<u8>, // TODO: Byte array of max 32767 length (should be inferred from packet length?).
}
