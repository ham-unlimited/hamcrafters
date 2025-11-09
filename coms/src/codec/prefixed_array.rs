use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::ser::{SerializeSeq, Serializer};
use std::fmt;

use crate::codec::var_int::VarInt;

/// An array with a varint length to be parsed
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrefixedArray<T>(Vec<T>);

impl<T> Serialize for PrefixedArray<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let length = self.0.len();
        let mut seq = serializer.serialize_seq(Some(length + 1))?;
        
        // Serialize length as VarInt
        seq.serialize_element(&VarInt(length as i32))?;
        
        // Serialize each element
        for item in &self.0 {
            seq.serialize_element(item)?;
        }
        
        seq.end()
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

                let mut items = Vec::with_capacity(length.0 as usize);

                for i in 0..length.0 {
                    let item: T = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i as usize + 1, &self))?;
                    items.push(item);
                }

                if seq.next_element::<de::IgnoredAny>()?.is_some() {
                    return Err(de::Error::custom("extra elements after the prefixed array"));
                }

                Ok(PrefixedArray(items))
            }
        }

        deserializer.deserialize_seq(PrefixedArrayVisitor {
            marker: std::marker::PhantomData,
        })
    }
}
