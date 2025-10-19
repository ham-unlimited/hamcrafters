use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

use crate::codec::prefixed_array::PrefixedArray;

/// Encryption request message
#[derive(Debug, Deserialize, Serialize)]
#[mc_packet(0x01)]
pub struct EncryptionRequest {
    server_id: String, // Empty for vanilla server
    public_key: PrefixedArray<u8>,
    verify_token: PrefixedArray<u8>,
    should_authenticate: bool,
}

impl EncryptionRequest {
    /// Creates a new [EncryptionRequest] with some defaults
    pub fn new(public_key: &String) -> Self {
        Self {
            server_id: "Hamcrafters".to_string(),
            public_key: PrefixedArray::from(public_key.bytes().collect::<Vec<u8>>()),
            // For security
            verify_token: PrefixedArray::from(vec![b'h', b'a', b'm']),
            should_authenticate: false,
        }
    }
}
