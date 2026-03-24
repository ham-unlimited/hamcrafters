use crate::{
    McPacket, codec::prefixed_array::PrefixedArray, messages::models::data_pack::DataPack,
};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Serverboudn known packs message.
#[derive(Debug, Deserialize)]
#[mc_packet(0x0E)]
pub struct ServerboundKnownPacks {
    known_packs: PrefixedArray<DataPack>,
}
