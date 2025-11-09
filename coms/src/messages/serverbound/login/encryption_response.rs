use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

use crate::codec::prefixed_array::PrefixedArray;

/// Encryption response message
#[derive(Debug, Deserialize, Serialize)]
#[mc_packet(0x01)]
pub struct EncryptionResponse {
    shared_secret: String,
    verify_token: PrefixedArray<u8>,
}
