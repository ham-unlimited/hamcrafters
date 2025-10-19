use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

use crate::codec::prefixed_array::PrefixedArray;

/// Encryption request message
#[derive(Debug, Deserialize)]
#[mc_packet(0x01)]
pub struct EncryptionRequest {
    server_id: String,
    public_key: PrefixedArray<u8>,
    verify_token: PrefixedArray<u8>,
    should_authenticate: bool,
}
