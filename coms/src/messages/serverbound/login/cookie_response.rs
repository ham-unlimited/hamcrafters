use crate::codec::prefixed_array::PrefixedArray;
use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// The cookie response msg
#[derive(Debug, Deserialize)]
#[mc_packet(0x04)]
pub struct CookieResponse {
    key: String,
    payload: Option<PrefixedArray<u8>>,
}
