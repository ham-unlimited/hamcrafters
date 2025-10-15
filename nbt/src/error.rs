use std::{io, num::TryFromIntError, string::FromUtf8Error};

/// An error that can occur in the crate.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum NbtError {
    #[error("IO Error `{0}`")]
    IoError(#[from] io::Error),
    #[error("Encountered invalid nbt tag {0}")]
    InvalidNbtTag(u8),
    #[error("Malformed NBT, err: {0}")]
    MalformedNbt(String),
    #[error("String contained invalid utf8, err: {0}")]
    InvalidUtf8(#[from] FromUtf8Error),
    #[error("Provided length was invalid, err: {0}")]
    InvalidLength(#[from] TryFromIntError),
}

/// Result type for the crate.
pub type NbtResult<T> = Result<T, NbtError>;
