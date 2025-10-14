use std::fmt::Display;

pub mod deserializer;
pub mod serializer;

#[macro_export]
macro_rules! unsupported {
    ($ty:literal) => {
        return Err(Error::Unsupported($ty))
    };
}

#[derive(Debug)]
pub enum Error {
    SerdeCustom(String),
    KeyWithoutValue,
    Unexpected(&'static str),
    Unsupported(&'static str),
    InvalidMapKey,
    MissingValueForKey,
    MissingKeyForValue,
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerdeCustom(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerdeCustom(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SerdeCustom(s) => f.write_str(s),
            Error::KeyWithoutValue => f.write_str("Tried to deserialize a key without a value?"),
            Error::Unexpected(msg) => f.write_str(&format!("Expected {msg}")),
            Error::Unsupported(type_name) => {
                f.write_str(&format!("Unsupported data type {type_name}"))
            }
            Error::InvalidMapKey => f.write_str("Invalid key for map"),
            Error::MissingKeyForValue => f.write_str("Got map value without a key"),
            Error::MissingValueForKey => f.write_str("Got map key without a value"),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;
