use serde::{Deserialize, Serialize};
use serde::Deserializer;
use serde::de;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::{SerializeSeq, Serializer};
use std::fmt;

/// The present variable decides if the data is present
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrefixedOptional<T>(Option<T>);

impl<T> Serialize for PrefixedOptional<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Some(value) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&true)?;
                seq.serialize_element(value)?;
                seq.end()
            }
            None => {
                let mut seq = serializer.serialize_seq(Some(1))?;
                seq.serialize_element(&false)?;
                seq.end()
            }
        }
    }
}

impl<'de, T> Deserialize<'de> for PrefixedOptional<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PrefixedOptionalVisitor<T> {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for PrefixedOptionalVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = PrefixedOptional<T>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "a prefixed optional: a boolean indicating whether the value is present, followed by the value if present"
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let present: bool = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let Some(v) = seq.next_element()? else {
                    if !present {
                        return Ok(PrefixedOptional(None));
                    }

                    return Err(de::Error::custom(
                        "Expected element, should exist because of prefix boolean but data not present",
                    ));
                };

                if !present {
                    return Err(de::Error::custom(
                        "Did not expect an element, but found one.",
                    ));
                }

                // TODO: maybe check for extra elements lol :3

                Ok(PrefixedOptional(v))
            }
        }

        deserializer.deserialize_seq(PrefixedOptionalVisitor {
            marker: std::marker::PhantomData,
        })
    }
}
