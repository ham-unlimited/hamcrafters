use std::fmt::Display;

pub mod deserializer;

#[derive(Debug)]
pub enum Error {
    SerdeCustom(String),
    KeyWithoutValue,
    Unexpected(&'static str),
    Unsupported(&'static str),
}

impl serde::de::StdError for Error {}

impl serde::de::Error for Error {
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
        }
    }
}

type Result<T> = std::result::Result<T, Error>;
