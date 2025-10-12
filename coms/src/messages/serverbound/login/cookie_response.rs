use crate::McPacket;
use crate::codec::{prefixed_array::PrefixedArray, prefixed_optional::PrefixedOptional};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[mc_packet(0x04)]
pub struct CookieResponse {
    pub key: String,
    pub payload: PrefixedOptional<PrefixedArray<u8>>,
}
