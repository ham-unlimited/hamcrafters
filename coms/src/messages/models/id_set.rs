use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeSeq};

use crate::codec::{identifier::Identifier, var_int::VarInt};

/// A minecraft registry ID set.
#[derive(Debug)]
pub enum IdSet {
    /// A registry tag, defining the ID set.
    TagName(Identifier),
    /// A list of registry IDs.
    IDs(Vec<VarInt>),
}

impl<'de> Deserialize<'de> for IdSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct IdSetVisitor;

        impl<'de> Visitor<'de> for IdSetVisitor {
            type Value = IdSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an IdSet")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let id_set_type: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                if id_set_type.0 == 0 {
                    let tag_name: Identifier = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                    Ok(IdSet::TagName(tag_name))
                } else {
                    let ids_len = id_set_type.0 as usize - 1;
                    let mut ids = Vec::with_capacity(ids_len);
                    for _ in 0..ids_len {
                        let id: VarInt = seq.next_element()?.ok_or_else(|| {
                            serde::de::Error::invalid_length(ids.len() + 1, &self)
                        })?;
                        ids.push(id);
                    }
                    Ok(IdSet::IDs(ids))
                }
            }
        }

        deserializer.deserialize_seq(IdSetVisitor)
    }
}

impl Serialize for IdSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            IdSet::TagName(tag_name) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(&VarInt(0))?;
                seq.serialize_element(&tag_name.0)?;
                seq.end()
            }
            IdSet::IDs(ids) => {
                let mut seq = serializer.serialize_seq(Some(ids.len() + 1))?;
                seq.serialize_element(&VarInt(ids.len() as i32 + 1))?;
                for id in ids {
                    seq.serialize_element(id)?;
                }
                seq.end()
            }
        }
    }
}
