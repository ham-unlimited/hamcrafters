use std::fmt::Display;

/// Deserialize implementation for NbtValue.
pub mod deserialize;
/// Deserializer implementation for NbtValue.
pub mod deserializer;
/// The actual NbtValue type.
pub mod nbt_value;

/// Macro for handling cases that are not supported by the deserializer & serializer.
#[macro_export]
macro_rules! unsupported_value {
    ($ty:literal) => {
        return Err(Self::Error::Unsupported($ty))
    };
}

/// An error that can occur during NbtValue operations.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum NbtValueError {
    SerdeCustom(String),
    Unsupported(&'static str),
    Unexpected(&'static str),
    KeyWithoutValue,
}

impl Display for NbtValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", match self {
            NbtValueError::SerdeCustom(c) => format!("custom: {c}"),
            NbtValueError::Unexpected(u) => format!("unexpected: {u}"),
            NbtValueError::Unsupported(u) => format!("unsupported: {u}"),
            NbtValueError::KeyWithoutValue => format!("Key without value"),
        }))
    }
}

impl serde::de::Error for NbtValueError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        NbtValueError::SerdeCustom(msg.to_string())
    }
}

impl serde::de::StdError for NbtValueError {}
