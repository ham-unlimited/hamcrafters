use std::{
    collections::HashMap,
    fmt::{self, Display},
    io::Read,
};

use serde::de::{self, Visitor};

use crate::{
    NbtResult,
    error::NbtError,
    nbt_named_tag::NbtNamedTag,
    nbt_types::{
        NbtByte, NbtByteArray, NbtCompound, NbtDouble, NbtFloat, NbtInt, NbtIntArray, NbtList,
        NbtLong, NbtLongArray, NbtShort, NbtString, NbtType,
    },
};

/// The different tags in NBT with the contained payloads.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub enum NbtTagType {
    TagEnd,
    TagByte(NbtByte),
    TagShort(NbtShort),
    TagInt(NbtInt),
    TagLong(NbtLong),
    TagFloat(NbtFloat),
    TagDouble(NbtDouble),
    TagByteArray(NbtByteArray),
    TagString(NbtString),
    TagList(NbtList),
    TagCompound(NbtCompound),
    TagIntArray(NbtIntArray),
    TagLongArray(NbtLongArray),
}

impl NbtTagType {
    /// Parse an NbtTagType from the provided [r].
    pub fn read<R: Read>(tag_id: u8, r: &mut R) -> NbtResult<Self> {
        Ok(match tag_id {
            0 => NbtTagType::TagEnd,
            1 => NbtTagType::TagByte(NbtByte::read(r)?),
            2 => NbtTagType::TagShort(NbtShort::read(r)?),
            3 => NbtTagType::TagInt(NbtInt::read(r)?),
            4 => NbtTagType::TagLong(NbtLong::read(r)?),
            5 => NbtTagType::TagFloat(NbtFloat::read(r)?),
            6 => NbtTagType::TagDouble(NbtDouble::read(r)?),
            7 => NbtTagType::TagByteArray(NbtByteArray::read(r)?),
            8 => NbtTagType::TagString(NbtString::read(r)?),
            9 => NbtTagType::TagList(NbtList::read(r)?),
            10 => NbtTagType::TagCompound(NbtCompound::read(r)?),
            11 => NbtTagType::TagIntArray(NbtIntArray::read(r)?),
            12 => NbtTagType::TagLongArray(NbtLongArray::read(r)?),
            b => return Err(NbtError::InvalidNbtTag(b)),
        })
    }

    /// Get the ID for this tag.
    pub fn get_tag_id(&self) -> u8 {
        match self {
            NbtTagType::TagEnd => 0,
            NbtTagType::TagByte(_) => 1,
            NbtTagType::TagShort(_) => 2,
            NbtTagType::TagInt(_) => 3,
            NbtTagType::TagLong(_) => 4,
            NbtTagType::TagFloat(_) => 5,
            NbtTagType::TagDouble(_) => 6,
            NbtTagType::TagByteArray(_) => 7,
            NbtTagType::TagString(_) => 8,
            NbtTagType::TagList(_) => 9,
            NbtTagType::TagCompound(_) => 10,
            NbtTagType::TagIntArray(_) => 11,
            NbtTagType::TagLongArray(_) => 12,
        }
    }

    // TODO: Finish
    // fn write<W: Write>(&self, w: &mut W) -> NbtResult<()> {
    //     let b: u8 = self.get_tag_id();
    //     w.write(&[b])?;

    //     match self {
    //         NbtTagType::TagEnd => {}
    //         t => todo!("Writing of nbttype {t:?} not implemented"),
    //     }

    //     Ok(())
    // }
}

/// Visitor for deserializing NbtTagType from any valid NBT tag.
pub struct NbtTagTypeVisitor;

impl<'de> Visitor<'de> for NbtTagTypeVisitor {
    type Value = NbtTagType;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("any valid NBT tag")
    }

    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagByte(NbtByte(if v { 1 } else { 0 })))
    }

    fn visit_i8<E: de::Error>(self, v: i8) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagByte(NbtByte(v)))
    }

    fn visit_i16<E: de::Error>(self, v: i16) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagShort(NbtShort(v)))
    }

    fn visit_i32<E: de::Error>(self, v: i32) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagInt(NbtInt(v)))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagLong(NbtLong(v)))
    }

    fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagByte(NbtByte(v as i8)))
    }

    fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagShort(NbtShort(v as i16)))
    }

    fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagInt(NbtInt(v as i32)))
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagLong(NbtLong(v as i64)))
    }

    fn visit_f32<E: de::Error>(self, v: f32) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagFloat(NbtFloat(v)))
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagDouble(NbtDouble(v)))
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagString(NbtString(v.to_owned())))
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(NbtTagType::TagString(NbtString(v)))
    }

    fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let tag_id: u8 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("expected tag id"))?;

        match tag_id {
            0 => Ok(NbtTagType::TagEnd),
            1 => {
                let v: i8 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected byte payload"))?;
                Ok(NbtTagType::TagByte(NbtByte(v)))
            }
            2 => {
                let v: i16 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected short payload"))?;
                Ok(NbtTagType::TagShort(NbtShort(v)))
            }
            3 => {
                let v: i32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected int payload"))?;
                Ok(NbtTagType::TagInt(NbtInt(v)))
            }
            4 => {
                let v: i64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected long payload"))?;
                Ok(NbtTagType::TagLong(NbtLong(v)))
            }
            5 => {
                let v: f32 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected float payload"))?;
                Ok(NbtTagType::TagFloat(NbtFloat(v)))
            }
            6 => {
                let v: f64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected double payload"))?;
                Ok(NbtTagType::TagDouble(NbtDouble(v)))
            }
            7 => {
                let v: Vec<i8> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected byte array payload"))?;
                Ok(NbtTagType::TagByteArray(NbtByteArray(
                    v.into_iter().map(NbtByte).collect(),
                )))
            }
            8 => {
                let v: String = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected string payload"))?;
                Ok(NbtTagType::TagString(NbtString(v)))
            }
            9 => {
                let v: Vec<NbtTagType> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected list payload"))?;
                Ok(NbtTagType::TagList(NbtList(v)))
            }
            10 => {
                let map: HashMap<String, NbtTagType> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected compound payload"))?;
                let entries = map
                    .into_iter()
                    .map(|(k, v)| NbtNamedTag {
                        name: NbtString(k),
                        payload: v,
                    })
                    .collect();
                Ok(NbtTagType::TagCompound(NbtCompound(entries)))
            }
            11 => {
                let v: Vec<i32> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected int array payload"))?;
                Ok(NbtTagType::TagIntArray(NbtIntArray(
                    v.into_iter().map(NbtInt).collect(),
                )))
            }
            12 => {
                let v: Vec<i64> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("expected long array payload"))?;
                Ok(NbtTagType::TagLongArray(NbtLongArray(
                    v.into_iter().map(NbtLong).collect(),
                )))
            }
            b => Err(de::Error::custom(format!("invalid tag id {b}"))),
        }
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut entries = Vec::new();
        while let Some((k, v)) = map.next_entry::<String, NbtTagType>()? {
            entries.push(NbtNamedTag {
                name: NbtString(k),
                payload: v,
            });
        }
        Ok(NbtTagType::TagCompound(NbtCompound(entries)))
    }
}

impl<'de> serde::Deserialize<'de> for NbtTagType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_seq(NbtTagTypeVisitor)
    }
}

impl Display for NbtTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}",
            match self {
                NbtTagType::TagEnd => "".to_string(),
                NbtTagType::TagByte(nbt_byte) => nbt_byte.0.to_string(),
                NbtTagType::TagShort(nbt_short) => nbt_short.0.to_string(),
                NbtTagType::TagInt(nbt_int) => nbt_int.0.to_string(),
                NbtTagType::TagLong(nbt_long) => nbt_long.0.to_string(),
                NbtTagType::TagFloat(nbt_float) => nbt_float.0.to_string(),
                NbtTagType::TagDouble(nbt_double) => nbt_double.0.to_string(),
                NbtTagType::TagByteArray(nbt_byte_array) => {
                    let s: Vec<String> = nbt_byte_array.0.iter().map(|b| b.0.to_string()).collect();
                    format!("[{}]", s.join(", "))
                }
                NbtTagType::TagString(nbt_string) => nbt_string.0.clone(),
                NbtTagType::TagList(nbt_list) => {
                    let s: Vec<String> = nbt_list.0.iter().map(|b| b.to_string()).collect();
                    format!("[{}]", s.join(", "))
                }
                NbtTagType::TagCompound(nbt_compound) => {
                    let s: Vec<String> = nbt_compound
                        .0
                        .iter()
                        .map(|tag| format!("{}: {}", tag.name.0.clone(), tag.payload.to_string()))
                        .collect();
                    format!("{{{}}}", s.join(", "))
                }
                NbtTagType::TagIntArray(nbt_int_array) => {
                    let s: Vec<String> = nbt_int_array.0.iter().map(|b| b.0.to_string()).collect();
                    format!("[{}]", s.join(", "))
                }
                NbtTagType::TagLongArray(nbt_long_array) => {
                    let s: Vec<String> = nbt_long_array.0.iter().map(|b| b.0.to_string()).collect();
                    format!("[{}]", s.join(", "))
                }
            }
        ))
    }
}
