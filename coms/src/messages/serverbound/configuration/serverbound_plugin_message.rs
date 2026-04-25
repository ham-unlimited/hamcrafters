use std::io::{self, Cursor, Read};

use crate::{
    codec::{identifier::Identifier, var_int::VarInt},
    messages::{McPacketError, McPacketRead},
    McPacket,
};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// A Minecraft serverbound plugin message packet.
#[derive(Debug)]
#[mc_packet(0x2)]
pub struct ServerboundPluginMessage {
    /// Name of the plugin channel used to send this message.
    pub channel: Identifier,
    /// Remaining data.
    pub data: Vec<u8>,
}

impl McPacketRead for ServerboundPluginMessage {
    type Output = Self;

    fn read(raw_packet: crate::packet_reader::RawPacket) -> Result<Self::Output, McPacketError> {
        let mut cursor = Cursor::new(raw_packet.data);

        let channel = Identifier::decode(&mut cursor)?;

        let mut data = Vec::new();
        cursor.read_to_end(&mut data)?;

        Ok(Self { channel, data })
    }
}
