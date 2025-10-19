use rsa::{RsaPrivateKey, traits::PublicKeyParts};

/// Encryption errors.
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum EncryptionError {
    #[error("Failed to generate private key: {0}")]
    PrivateKeyGenFailure(#[from] rsa::Error),
}

/// A keystore for keeping minecraft encryption keys.
pub struct KeyStore {
    private_key: RsaPrivateKey,
}

const MINECRAFT_RSA_KEY_SIZE: usize = 1024;

impl KeyStore {
    /// Create a new [KeyStore], generating a new keyair in the process.
    #[must_use]
    pub fn new() -> Result<Self, EncryptionError> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, MINECRAFT_RSA_KEY_SIZE)?;

        Ok(Self { private_key })
    }

    /// Get a copy of the public key in der format.
    pub fn get_der_public_key(&self) -> Vec<u8> {
        rsa_der::public_key_to_der(
            self.private_key.n().to_bytes_be().as_slice(),
            self.private_key.e().to_bytes_be().as_slice(),
        )
    }
}
