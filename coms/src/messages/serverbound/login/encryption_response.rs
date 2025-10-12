use mc_packet::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

use crate::codec::prefixed_array::PrefixedArray;

#[derive(Debug, Deserialize)]
#[mc_packet(0x01)]
pub struct EncryptionResponse {
    pub shared_secret: String,
    pub verify_token: PrefixedArray<u8>,
}
