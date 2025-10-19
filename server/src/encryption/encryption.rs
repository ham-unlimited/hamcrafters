use rsa::{RsaPrivateKey, RsaPublicKey};

use asn1_der::{DerObject, typed::DerEncodable};

/// Encryption keys for the Minecraft protocol.
/// This is used to encrypt the packets sent to the client.
#[derive(Debug)]
pub struct McEncryptionKeys {
    /// The public key of the encryption keys.
    pub pub_key: RsaPublicKey,
    /// The private key of the encryption keys.
    pub priv_key: RsaPrivateKey,
}

impl McEncryptionKeys {
    /// Generates new encryption keys.
    pub fn new() -> McEncryptionKeys {
        let mut rng = rand::thread_rng();
        let bits = 2048;

        let priv_key = RsaPrivateKey::new(&mut rng, bits)?;
        let pub_key = RsaPublicKey::from(&priv_key);
        McEncryptionKeys { pub_key, priv_key }
    }

    /// DER encode public key
    pub fn der_encode_pub_key(&self) -> String {
        let mut der_encoded_key = String::new();
        String::encode(&self.pub_key, &mut der_encoded_key);
        der_encoded_key
    }
}
