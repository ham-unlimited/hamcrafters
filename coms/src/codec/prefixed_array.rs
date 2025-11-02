use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

use crate::codec::var_int::VarInt;

/// An array with a varint length to be parsed
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrefixedArray<T>(Vec<T>);

impl<T> PrefixedArray<T> {
    /// Get a reference to contained vector.
    pub fn inner(&self) -> &Vec<T> {
        &self.0
    }
}

impl<'de, T> Deserialize<'de> for PrefixedArray<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PrefixedArrayVisitor<T> {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for PrefixedArrayVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = PrefixedArray<T>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "a prefixed array: length as a varint followed by that many items"
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let length: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                log::info!("LENGTH {length:?}");

                let mut items = Vec::with_capacity(length.0 as usize);

                // let mut items = Vec::new();

                // while let Some(item) = seq.next_element()? {
                //     items.push(item);
                // }

                for i in 0..length.0 {
                    let item: T = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i as usize + 1, &self))?;
                    items.push(item);
                }

                // TODO: Check if there are more elements.

                Ok(PrefixedArray(items))
            }
        }

        deserializer.deserialize_seq(PrefixedArrayVisitor {
            marker: std::marker::PhantomData,
        })
    }
}

impl<T: Serialize + Debug> Serialize for PrefixedArray<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;

        for elem in self.0.iter() {
            seq.serialize_element(elem)?;
        }

        seq.end()
    }
}

impl<T> From<Vec<T>> for PrefixedArray<T> {
    fn from(value: Vec<T>) -> Self {
        PrefixedArray(value)
    }
}
