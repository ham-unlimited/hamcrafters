use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

use crate::codec::prefixed_array::PrefixedArray;

/// Encryption response message
#[derive(Debug, Serialize, Deserialize)]
#[mc_packet(0x01)]
pub struct EncryptionResponse {
    /// Symmetric key to be used to encrypt/decrypt future communication, encrypted using the servers public key.
    pub shared_secret: PrefixedArray<u8>,
    /// A verification token encrypted using the shared secret.
    pub verify_token: PrefixedArray<u8>,
}
