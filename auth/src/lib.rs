#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Crate for handling minecraft auth.

use std::time::Duration;

use log::error;
use num_bigint::{BigInt, Sign};
use reqwest::{ClientBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use uuid::Uuid;

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Failed to send HTTP request {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error response from microsoft servers")]
    AuthFailure {
        error: String,
        path: String,
        message: String,
    },
}

/// Result type for the auth lib.
pub type AuthResult<T> = Result<T, AuthError>;

const MINECRAFT_AUTH_URI: &'static str = "https://sessionserver.mojang.com/session/minecraft/join";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientAuthRequest {
    access_token: String,
    selected_profile: String,
    server_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MinecraftAuthErrorResponse {
    error: String,
    path: String,
    error_message: Option<String>,
}

/// Performs client auth based on the provided parameters.
pub async fn client_auth(
    access_token: String,
    player_id: Uuid,
    server_id: String,
    shared_secret: String,
    encoded_public_key: String,
) -> AuthResult<()> {
    let complete_base = format!("{server_id}{shared_secret}{encoded_public_key}");
    let sha1 = get_sha1(&complete_base);

    let selected_profile = player_id.to_string().replace("-", "");

    let client = ClientBuilder::new()
        .connect_timeout(Duration::from_secs(30))
        .build()?;

    let response = client
        .get(MINECRAFT_AUTH_URI)
        .json(&ClientAuthRequest {
            access_token,
            selected_profile,
            server_id: sha1,
        })
        .send()
        .await?;

    let status = response.status();
    if status == StatusCode::NO_CONTENT {
        return Ok(());
    }

    let error: MinecraftAuthErrorResponse = response.json().await?;

    let message = error.error_message.unwrap_or("".to_string());
    error!(
        "Client Auth failed with status {} (\"{message}\"), error {} at path {}",
        status.as_str(),
        error.error,
        error.path
    );

    Err(AuthError::AuthFailure {
        error: error.error,
        path: error.path,
        message,
    })
}

fn get_sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());

    let output = hasher.finalize();

    let bigint = BigInt::from_signed_bytes_be(&output);
    let output = if bigint.sign() == Sign::Minus {
        format!("-{:x}", (-bigint))
    } else {
        format!("{:x}", bigint)
    };

    output
}

#[cfg(test)]
mod tests {
    use crate::get_sha1;

    #[test]
    fn test_notch() {
        assert_eq!(
            get_sha1("Notch"),
            "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48".to_string()
        )
    }

    #[test]
    fn test_jeb() {
        assert_eq!(
            get_sha1("jeb_"),
            "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1".to_string()
        )
    }

    #[test]
    fn test_simon() {
        assert_eq!(
            get_sha1("simon"),
            "88e16a1019277b15d58faf0541e11910eb756f6".to_string()
        )
    }
}
