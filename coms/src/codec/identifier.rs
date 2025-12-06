use std::{fmt::Display, io::Read, num::NonZeroUsize};

use serde::{self, Deserialize};

use crate::{codec::var_int::VarInt, ser::ReadingError};

/**
 * An identifier on the form minecraft:thing, implemented as a String prefixed by its length as a VarInt.
 */
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Identifier(pub String);

impl Identifier {
    // TODO: This limit is currently not enforced when Deserializing this type with serde.
    const MAX_LENGTH: NonZeroUsize = NonZeroUsize::new(32767).unwrap();

    /// Read an Identifier from the provided [read].
    pub fn decode(read: &mut impl Read) -> Result<Self, ReadingError> {
        let identifier_length = VarInt::decode(read)?;
        if identifier_length.0 > (Self::MAX_LENGTH.get() as i32) {
            return Err(ReadingError::TooLarge(format!(
                "Identifier length was too large {} > {}",
                identifier_length.0,
                Self::MAX_LENGTH.get()
            )));
        }

        let mut identifier = vec![0; identifier_length.0 as usize];
        read.read_exact(&mut identifier)?;
        let identifier = String::from_utf8(identifier)?;

        Ok(Self(identifier))
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
