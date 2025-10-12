use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

use crate::codec::prefixed_array::PrefixedArray;

#[derive(Debug, Deserialize)]
#[mc_packet(0x01)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: PrefixedArray<u8>,
    pub verify_token: PrefixedArray<u8>,
    pub should_authenticate: bool,
}
