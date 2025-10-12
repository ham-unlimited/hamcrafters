use serde::Deserialize;
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use std::fmt;

use crate::codec::var_int::VarInt;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrefixedArray<T> {
    length: VarInt,
    data: Vec<T>,
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

                let mut items = Vec::with_capacity(length.0 as usize);

                for i in 0..length.0 {
                    let item: T = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i as usize + 1, &self))?;
                    items.push(item);
                }

                if let Some(_) = seq.next_element::<de::IgnoredAny>()? {
                    return Err(de::Error::custom("extra elements after the prefixed array"));
                }

                Ok(PrefixedArray {
                    length,
                    data: items,
                })
            }
        }

        deserializer.deserialize_seq(PrefixedArrayVisitor {
            marker: std::marker::PhantomData,
        })
    }
}
