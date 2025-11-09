use crate::{McPacket, codec::var_int::VarInt};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Information about the clients settings.
#[derive(Debug, Deserialize)]
#[mc_packet(0x0)]
#[serde(rename_all = "PascalCase")]
pub struct ClientInformation {
    locale: String,
    view_distance: i8,
    chat_mode: VarInt, // TODO: Really an enum.
    chat_colors: bool,
    displayed_skin_parts: u8, // TODO: Bitmask (maybe use a struct?)
    main_hand: VarInt,        // TODO: Enum
    enable_text_filtering: bool,
    allow_server_listings: bool,
    particle_status: VarInt, // TODO: Enum.
}
