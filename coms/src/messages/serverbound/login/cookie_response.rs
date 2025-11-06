use crate::McPacket;
use crate::codec::{prefixed_array::PrefixedArray, prefixed_optional::PrefixedOptional};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// The cookie response msg
#[derive(Debug, Deserialize, Serialize)]
#[mc_packet(0x04)]
pub struct CookieResponse {
    key: String,
    payload: PrefixedOptional<PrefixedArray<u8>>,
}
