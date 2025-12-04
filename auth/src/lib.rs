#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Crate for handling minecraft auth.

use num_bigint::{BigInt, Sign};
use sha1::{Digest, Sha1};

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AuthError {
    // #[error("")]
    // HashError(#[from] sha1::Sha1::Error),
}

/// Result type for the auth lib.
pub type AuthResult<T> = Result<T, AuthError>;

fn get_sha1(input: &str) -> AuthResult<String> {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());

    let output = hasher.finalize();

    let bigint = BigInt::from_signed_bytes_be(&output);
    let output = if bigint.sign() == Sign::Minus {
        format!("-{:x}", (-bigint))
    } else {
        format!("{:x}", bigint)
    };

    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::get_sha1;

    #[test]
    fn test_notch() {
        assert_eq!(
            get_sha1("Notch"),
            Ok("4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48".to_string())
        )
    }

    #[test]
    fn test_jeb() {
        assert_eq!(
            get_sha1("jeb_"),
            Ok("-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1".to_string())
        )
    }
}
