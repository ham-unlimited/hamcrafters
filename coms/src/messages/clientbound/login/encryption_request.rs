use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

use crate::codec::prefixed_array::PrefixedArray;

/// Encryption request message
#[derive(Debug, Deserialize, Serialize)]
#[mc_packet(0x01)]
pub struct EncryptionRequest {
    /// An identify of the server, empty for a usual vanilla server.
    pub server_id: String,
    /// Servers public key.
    pub public_key: PrefixedArray<u8>,
    /// A verification token to ensure that the [EncryptionResponse] is correctly encrypted.
    pub verify_token: PrefixedArray<u8>,
    /// Weather the client (and server) should validate the session against Mojang servers.
    pub should_authenticate: bool,
}

impl EncryptionRequest {
    /// Creates a new [EncryptionRequest] with some defaults
    pub fn new(der_public_key: Vec<u8>) -> Self {
        Self {
            server_id: "Hamcrafters".to_string(),
            public_key: PrefixedArray::from(der_public_key),
            // For security
            verify_token: PrefixedArray::from(vec![b'h', b'a', b'm']),
            should_authenticate: false,
        }
    }
}
