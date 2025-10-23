use serde::{Deserialize, de::Visitor};

use crate::{
    nbt_named_tag::NbtNamedTag,
    nbt_types::{
        NbtByte, NbtByteArray, NbtCompound, NbtDouble, NbtFloat, NbtInt, NbtList, NbtLong,
        NbtShort, NbtString,
    },
    tag_type::NbtTagType,
};

//! Generic NBT deserialized value, intended to function similar to serde_json's Value type.

pub enum NbtValue {
    
}

struct NbtValueVisitor;

impl<'de> Visitor<'de> for NbtValueVisitor {
    type Value = NbtTagType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Any valid NBTTagType")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagByte(NbtByte(if v { 1 } else { 0 })))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagByte(NbtByte(v)))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagShort(NbtShort(v)))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagInt(NbtInt(v)))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagLong(NbtLong(v)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagByte(NbtByte(v as i8)))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagShort(NbtShort(v as i16)))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagInt(NbtInt(v as i32)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagLong(NbtLong(v as i64)))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagFloat(NbtFloat(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagDouble(NbtDouble(v)))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagByte(NbtByte(v as i8)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagString(NbtString(String::from(v))))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagString(NbtString(v)))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagByteArray(NbtByteArray(
            v.into_iter().map(|b| NbtByte(*b as i8)).collect(),
        )))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NbtTagType::TagByteArray(NbtByteArray(
            v.into_iter().map(|b| NbtByte(b as i8)).collect(),
        )))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = seq.next_element()? {
            vec.push(elem);
        }

        Ok(NbtTagType::TagList(NbtList(vec)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut vec = Vec::new();

        while let Some((key, value)) = map.next_entry()? {
            vec.push(NbtNamedTag {
                name: NbtString(key),
                payload: value,
            })
        }

        Ok(NbtTagType::TagCompound(NbtCompound(vec)))
    }
}

impl<'de> Deserialize<'de> for NbtValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(NbtValueVisitor)
    }
}
