use crate::{
    McPacket,
    codec::{prefixed_array::PrefixedArray, prefixed_optional::PrefixedOptional, var_int::VarInt},
    messages::models::text_component::TextComponent,
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound server data packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x7d)]
pub struct SetTickingRate {
    /// Tick rate in ticks per second.
    pub tick_rate: f32,
    /// Whether the ticking is frozen or not.
    pub is_frozen: bool,
}
