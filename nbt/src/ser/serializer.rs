use serde::ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct};
use serde::{Serialize, ser};

use crate::nbt_named_tag::NbtNamedTag;
use crate::nbt_types::{
    NbtByte, NbtCompound, NbtDouble, NbtFloat, NbtInt, NbtList, NbtLong, NbtShort, NbtString,
};
use crate::ser::{Error, Result};
use crate::tag_type::NbtTagType;
use crate::unsupported;

pub struct Serializer {}

impl Serializer {
    fn new() -> Self {
        Self {}
    }
}

pub fn to_nbt_tag_type<T: Serialize>(value: &T) -> Result<Option<NbtTagType>> {
    let mut serializer = Serializer::new();
    let v = value.serialize(&mut serializer)?;

    Ok(v)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = Option<NbtTagType>;

    type Error = Error;

    type SerializeSeq = SeqSerializer;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = CompoundMapSerializer;
    type SerializeStruct = CompoundStructSerializer;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagByte(NbtByte(if v { 1 } else { 0 }))))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagByte(NbtByte(v))))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagShort(NbtShort(v))))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagInt(NbtInt(v))))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagLong(NbtLong(v))))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        unsupported!("u8")
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        unsupported!("u16")
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        unsupported!("u32")
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        unsupported!("u64")
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagFloat(NbtFloat(v))))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagDouble(NbtDouble(v))))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        unsupported!("char")
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagString(NbtString(v.to_string()))))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        unsupported!("bytes")
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(None)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer::new();
        Ok(value.serialize(&mut serializer)?)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        unsupported!("unit")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        unsupported!("unit_struct")
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        unsupported!("unit_variant")
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        unsupported!("newtype_struct")
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        unsupported!("newtype_variant")
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        let s = Self::SerializeSeq { values: Vec::new() };
        Ok(s)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unsupported!("tuple")
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unsupported!("tuple_struct")
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unsupported!("tuple_variant")
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Self::SerializeMap::new())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(Self::SerializeStruct::new())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unsupported!("struct_variant")
    }
}

pub struct SeqSerializer {
    values: Vec<NbtTagType>,
}

impl SerializeSeq for SeqSerializer {
    type Ok = Option<NbtTagType>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer::new();
        if let Some(o) = value.serialize(&mut serializer)? {
            self.values.push(o)
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagList(NbtList(self.values))))
    }
}

pub struct CompoundMapSerializer {
    new_name: Option<NbtString>,
    tags: Vec<NbtNamedTag>,
}

impl CompoundMapSerializer {
    fn new() -> Self {
        Self {
            new_name: None,
            tags: Vec::new(),
        }
    }
}

impl SerializeMap for CompoundMapSerializer {
    type Ok = Option<NbtTagType>;

    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer::new();
        let Some(NbtTagType::TagString(s)) = key.serialize(&mut serializer)? else {
            return Err(Error::InvalidMapKey);
        };

        println!("Serializing key for map {s:?}");

        self.new_name = Some(s);

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let Some(key) = self.new_name.take() else {
            return Err(Error::MissingValueForKey);
        };

        let mut serializer = Serializer::new();
        if let Some(o) = value.serialize(&mut serializer)? {
            self.tags.push(NbtNamedTag {
                name: key,
                payload: o,
            });
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        if self.new_name.is_some() {
            return Err(Error::KeyWithoutValue);
        }

        Ok(Some(NbtTagType::TagCompound(NbtCompound(self.tags))))
    }
}

pub struct CompoundStructSerializer {
    fields: Vec<NbtNamedTag>,
}

impl CompoundStructSerializer {
    fn new() -> Self {
        Self { fields: Vec::new() }
    }
}

impl SerializeStruct for CompoundStructSerializer {
    type Ok = Option<NbtTagType>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer::new();
        let value = value.serialize(&mut serializer)?;

        if let Some(value) = value {
            self.fields.push(NbtNamedTag {
                name: NbtString(key.to_string()),
                payload: value,
            })
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(Some(NbtTagType::TagCompound(NbtCompound(self.fields))))
    }
}
