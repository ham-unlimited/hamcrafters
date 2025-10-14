use serde::de::{
    self, IntoDeserializer, MapAccess,
    value::{SeqDeserializer, StrDeserializer},
};

use crate::{
    nbt_named_tag::NbtNamedTag,
    ser::{Error, Result},
    tag_type::NbtTagType,
    unsupported,
};

pub struct Deserializer {
    input: NbtTagType,
}

impl Deserializer {
    pub fn from_nbt_tag(input: NbtTagType) -> Self {
        Self { input }
    }
}

impl<'de> IntoDeserializer<'de, Error> for NbtTagType {
    type Deserializer = Deserializer;

    fn into_deserializer(self) -> Self::Deserializer {
        Deserializer::from_nbt_tag(self)
    }
}

struct CompoundVisitor {
    contents: Vec<NbtNamedTag>,
}

impl<'de> MapAccess<'de> for CompoundVisitor {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        let Some(tag) = self.contents.last() else {
            return Ok(None);
        };

        seed.deserialize(StrDeserializer::new(&tag.name.0))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let Some(tag) = self.contents.pop() else {
            return Err(Error::KeyWithoutValue);
        };

        seed.deserialize(Deserializer::from_nbt_tag(tag.payload))
    }
}

impl<'de> de::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match &self.input {
            NbtTagType::TagEnd => self.deserialize_unit(visitor),
            NbtTagType::TagByte(_) => self.deserialize_i8(visitor),
            NbtTagType::TagShort(_) => self.deserialize_i16(visitor),
            NbtTagType::TagInt(_) => self.deserialize_i32(visitor),
            NbtTagType::TagLong(_) => self.deserialize_i64(visitor),
            NbtTagType::TagFloat(_) => self.deserialize_f32(visitor),
            NbtTagType::TagDouble(_) => self.deserialize_f64(visitor),
            NbtTagType::TagByteArray(_) => self.deserialize_seq(visitor),
            NbtTagType::TagString(_) => self.deserialize_string(visitor),
            NbtTagType::TagList(_) => self.deserialize_seq(visitor),
            NbtTagType::TagCompound(_) => self.deserialize_map(visitor),
            NbtTagType::TagIntArray(_) => self.deserialize_seq(visitor),
            NbtTagType::TagLongArray(_) => self.deserialize_seq(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let NbtTagType::TagByte(i) = self.input else {
            return Err(Error::Unexpected("Expected boolean (byte)"));
        };

        let b = match i.0 {
            0 => true,
            1 => false,
            _ => return Err(Error::Unexpected("Expected valid bool value for byte")),
        };

        visitor.visit_bool(b)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let NbtTagType::TagByte(i) = self.input else {
            return Err(Error::Unexpected("Expected i8"));
        };

        visitor.visit_i8(i.0)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let NbtTagType::TagShort(i) = self.input else {
            return Err(Error::Unexpected("Expected i16"));
        };

        visitor.visit_i16(i.0)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let NbtTagType::TagInt(i) = self.input else {
            return Err(Error::Unexpected("Expected i32"));
        };

        visitor.visit_i32(i.0)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let NbtTagType::TagLong(i) = self.input else {
            return Err(Error::Unexpected("Expected i64"));
        };

        visitor.visit_i64(i.0)
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("u8")
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("u16")
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("u32")
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("u64")
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let NbtTagType::TagFloat(f) = self.input else {
            return Err(Error::Unexpected("Expected f32"));
        };

        visitor.visit_f32(f.0)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let NbtTagType::TagDouble(f) = self.input else {
            return Err(Error::Unexpected("Expected f64"));
        };

        visitor.visit_f64(f.0)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("char")
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("str")
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let NbtTagType::TagString(s) = self.input else {
            return Err(Error::Unexpected("Expected string"));
        };

        visitor.visit_string(s.0)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("bytes")
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("byte_buf")
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // Should only be called if the value exists.
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("unit")
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("unit_struct")
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("newtype_struct")
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.input {
            NbtTagType::TagByteArray(nbt_byte_array) => {
                visitor.visit_seq(SeqDeserializer::new(nbt_byte_array.0.iter().map(|v| v.0)))
            }
            NbtTagType::TagList(nbt_list) => {
                visitor.visit_seq(SeqDeserializer::new(nbt_list.0.into_iter()))
            }
            NbtTagType::TagIntArray(nbt_int_array) => {
                visitor.visit_seq(SeqDeserializer::new(nbt_int_array.0.iter().map(|v| v.0)))
            }
            NbtTagType::TagLongArray(nbt_long_array) => {
                visitor.visit_seq(SeqDeserializer::new(nbt_long_array.0.iter().map(|v| v.0)))
            }
            _ => Err(Error::Unexpected("Expected list")),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("tuple")
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("tuple_struct")
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let NbtTagType::TagCompound(comp) = self.input else {
            return Err(Error::Unexpected("Expected compound"));
        };

        visitor.visit_map(CompoundVisitor { contents: comp.0 })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("enum")
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unsupported!("identifier")
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}
