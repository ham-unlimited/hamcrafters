use std::collections::HashMap;

use serde::{
    Deserializer,
    de::{
        EnumAccess, IntoDeserializer, MapAccess, VariantAccess, Visitor,
        value::{SeqDeserializer, StrDeserializer, StringDeserializer},
    },
};

use crate::{
    nbt_value::{NbtValueError, nbt_value::NbtValue},
    unsupported_value,
};

impl<'de> IntoDeserializer<'de, NbtValueError> for NbtValue {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> Deserializer<'de> for NbtValue {
    type Error = NbtValueError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match &self {
            NbtValue::Byte(_byte) => self.deserialize_i8(visitor),
            NbtValue::Short(_short) => self.deserialize_i16(visitor),
            NbtValue::Int(_int) => self.deserialize_i32(visitor),
            NbtValue::Long(_long) => self.deserialize_i64(visitor),
            NbtValue::Float(_float) => self.deserialize_f32(visitor),
            NbtValue::Double(_double) => self.deserialize_f64(visitor),
            NbtValue::ByteArray(_byte_array) => self.deserialize_seq(visitor),
            NbtValue::String(_string) => self.deserialize_string(visitor),
            NbtValue::List(_values) => self.deserialize_seq(visitor),
            NbtValue::Compound(_fields) => self.deserialize_map(visitor),
            NbtValue::IntArray(_ints) => self.deserialize_seq(visitor),
            NbtValue::LongArray(_longs) => self.deserialize_seq(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Byte(i) = self else {
            return Err(Self::Error::Unexpected("Expected boolean (byte)"));
        };

        let b = match i {
            0 => true,
            1 => false,
            _ => {
                return Err(Self::Error::Unexpected(
                    "Expected valid bool value for byte",
                ));
            }
        };

        visitor.visit_bool(b)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Byte(i) = self else {
            return Err(Self::Error::Unexpected("Expected byte"));
        };

        visitor.visit_i8(i)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Short(i) = self else {
            return Err(Self::Error::Unexpected("Expected short"));
        };

        visitor.visit_i16(i)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Int(i) = self else {
            return Err(Self::Error::Unexpected("Expected int"));
        };

        visitor.visit_i32(i)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Long(i) = self else {
            return Err(Self::Error::Unexpected("Expected long"));
        };

        visitor.visit_i64(i)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Byte(i) = self else {
            return Err(Self::Error::Unexpected("Expected byte"));
        };

        visitor.visit_u8(i as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Short(i) = self else {
            return Err(Self::Error::Unexpected("Expected short"));
        };

        visitor.visit_u16(i as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Int(i) = self else {
            return Err(Self::Error::Unexpected("Expected int"));
        };

        visitor.visit_u32(i as u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Long(i) = self else {
            return Err(Self::Error::Unexpected("Expected long"));
        };

        visitor.visit_u64(i as u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Float(f) = self else {
            return Err(Self::Error::Unexpected("Expected float"));
        };

        visitor.visit_f32(f)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Double(f) = self else {
            return Err(Self::Error::Unexpected("Expected double"));
        };

        visitor.visit_f64(f)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Byte(b) = self else {
            return Err(Self::Error::Unexpected("Expected byte"));
        };

        visitor.visit_char((b as u8) as char)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::String(s) = self else {
            return Err(Self::Error::Unexpected("Expected string"));
        };

        visitor.visit_str(s.as_str())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::String(s) = self else {
            return Err(Self::Error::Unexpected("Expected string"));
        };

        visitor.visit_string(s)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            NbtValue::ByteArray(items) => {
                visitor.visit_seq(SeqDeserializer::new(items.into_iter()))
            }
            NbtValue::List(nbt_values) => {
                visitor.visit_seq(SeqDeserializer::new(nbt_values.into_iter()))
            }
            NbtValue::IntArray(items) => visitor.visit_seq(SeqDeserializer::new(items.into_iter())),
            NbtValue::LongArray(items) => {
                visitor.visit_seq(SeqDeserializer::new(items.into_iter()))
            }
            _ => Err(Self::Error::Unexpected("Expected list")),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let NbtValue::Compound(map) = self else {
            return Err(Self::Error::Unexpected("Expected compound"));
        };

        visitor.visit_map(NbtValueCompoundvisitor::new(map))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let hash_map = match self {
            NbtValue::String(s) => return visitor.visit_enum(s.into_deserializer()),
            NbtValue::Compound(hash_map) => hash_map,
            _ => {
                return Err(Self::Error::Unexpected("Expected compound for enum"));
            }
        };

        let mut iter = hash_map.into_iter();
        let (key, val) = match iter.next() {
            Some((k, v)) => (k, v),
            None => {
                return Err(Self::Error::Unexpected(
                    "Expected enum map to contain an entry",
                ));
            }
        };

        if iter.next().is_some() {
            return Err(Self::Error::Unexpected(
                "Expected enum map to only contain one value",
            ));
        }

        visitor.visit_enum(NbtValueEnumDeserializer::new(key, val))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unsupported_value!("Ignored any")
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unsupported_value!("Bytes")
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unsupported_value!("Byte buf")
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unsupported_value!("Option")
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unsupported_value!("Unit")
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unsupported_value!("Unit struct")
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unsupported_value!("Newtype struct")
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
}

struct NbtValueCompoundvisitor {
    contents: Vec<(String, NbtValue)>,
}

impl NbtValueCompoundvisitor {
    fn new(map: HashMap<String, NbtValue>) -> Self {
        Self {
            contents: map.into_iter().collect(),
        }
    }
}

impl<'de> MapAccess<'de> for NbtValueCompoundvisitor {
    type Error = NbtValueError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        let Some((key, _)) = self.contents.first() else {
            return Ok(None);
        };

        seed.deserialize(StrDeserializer::new(key)).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let Some((_, value)) = self.contents.pop() else {
            return Err(Self::Error::KeyWithoutValue);
        };

        seed.deserialize(value.into_deserializer())
    }
}

#[derive(Debug)]
struct NbtValueEnumDeserializer {
    key: String,
    val: NbtValue,
}

impl NbtValueEnumDeserializer {
    fn new(key: String, val: NbtValue) -> Self {
        Self { key, val }
    }
}

impl<'de> EnumAccess<'de> for NbtValueEnumDeserializer {
    type Error = NbtValueError;
    type Variant = NbtValueEnumVariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(StringDeserializer::new(self.key))
            .map(|v| (v, NbtValueEnumVariantDeserializer { val: self.val }))
    }
}

#[derive(Debug)]
struct NbtValueEnumVariantDeserializer {
    val: NbtValue,
}

impl<'de> VariantAccess<'de> for NbtValueEnumVariantDeserializer {
    type Error = NbtValueError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        // Should have been handled in deserialize_enum.
        unsupported_value!("Enum Access unit variant should have been handled earlier")
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.val)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Deserializer::deserialize_seq(self.val, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Deserializer::deserialize_map(self.val, visitor)
    }
}
