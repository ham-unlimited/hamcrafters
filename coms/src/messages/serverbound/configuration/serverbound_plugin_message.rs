use std::io::{self, Cursor, Read};

use crate::{
    McPacket,
    codec::var_int::VarInt,
    messages::{McPacketError, McPacketRead},
};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// A Minecraft serverbound plugin message packet.
#[derive(Debug)]
#[mc_packet(0x2)]
pub struct ServerboundPluginMessage {
    /// Name of the plugin channel used to send this message.
    pub channel: String, // TODO: Really an identifier
    /// Remaining data.
    pub data: Vec<u8>,
}

impl McPacketRead for ServerboundPluginMessage {
    type Output = Self;

    fn read(raw_packet: crate::packet_reader::RawPacket) -> Result<Self::Output, McPacketError> {
        let mut cursor = Cursor::new(raw_packet.data);

        let channel_length = VarInt::decode(&mut cursor)?;
        let mut channel = vec![0; channel_length.0 as usize];
        cursor.read_exact(&mut channel)?;
        let channel = String::from_utf8(channel)?;

        let mut data = Vec::new();
        cursor.read_to_end(&mut data)?;

        Ok(Self { channel, data })
    }
}
