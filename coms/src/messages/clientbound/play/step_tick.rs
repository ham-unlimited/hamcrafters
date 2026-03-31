use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::{id_set::IdSet, slot_display::SlotDisplay},
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound step tick packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x7E)]
pub struct StepTick {
    /// The number of tick steps.
    pub tick_steps: VarInt,
}
