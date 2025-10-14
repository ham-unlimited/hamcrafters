use std::io::Read;

use crate::{NbtError, NbtResult, nbt_named_tag::NbtNamedTag, tag_type::NbtTagType};

pub trait NbtType {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self>
    where
        Self: Sized;

    fn to_tag_type(self) -> NbtTagType;
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtByte(pub i8);

impl NbtType for NbtByte {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 1];
        r.read_exact(&mut bs)?;
        Ok(Self(i8::from_be_bytes(bs)))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagByte(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtShort(pub i16);

impl NbtType for NbtShort {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 2];
        r.read_exact(&mut bs)?;
        Ok(Self(i16::from_be_bytes(bs)))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagShort(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtInt(pub i32);

impl NbtType for NbtInt {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 4];
        r.read_exact(&mut bs)?;
        Ok(Self(i32::from_be_bytes(bs)))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagInt(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtLong(pub i64);

impl NbtType for NbtLong {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 8];
        r.read_exact(&mut bs)?;
        Ok(Self(i64::from_be_bytes(bs)))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagLong(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtFloat(pub f32);

impl NbtType for NbtFloat {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 4];
        r.read_exact(&mut bs)?;
        Ok(Self(f32::from_be_bytes(bs)))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagFloat(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtDouble(pub f64);

impl NbtType for NbtDouble {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 8];
        r.read_exact(&mut bs)?;
        Ok(Self(f64::from_be_bytes(bs)))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagDouble(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct NbtByteArray(pub Vec<NbtByte>);

impl NbtType for NbtByteArray {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let length = NbtInt::read(r)?;
        let length = usize::try_from(length.0)?;

        let mut buffer = Vec::with_capacity(length * size_of::<NbtByte>());
        for _ in 0..length {
            buffer.push(NbtByte::read(r)?);
        }
        Ok(Self(buffer))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagByteArray(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtString(pub String);

impl NbtType for NbtString {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let length = NbtShort::read(r)?;
        let length = usize::try_from(length.0)?;

        let mut string_buffer = vec![0; length];
        r.read_exact(&mut string_buffer)?;

        let s = String::from_utf8(string_buffer)?;

        Ok(Self(s))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagString(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtList(pub Vec<NbtTagType>);

impl NbtType for NbtList {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let tag_id = NbtByte::read(r)?;
        let tag_id = u8::try_from(tag_id.0)?;

        let length = NbtInt::read(r)?;
        let length = usize::try_from(length.0)?;

        let mut buffer = Vec::new();
        for _ in 0..length {
            let t = NbtTagType::read(tag_id, r)?;
            buffer.push(t);
        }

        if let Some(t) = buffer.iter().find(|e| e.get_tag_id() != tag_id) {
            return Err(NbtError::MalformedNbt(format!(
                "List of type {tag_id} contained element with id {}",
                t.get_tag_id()
            )));
        }

        Ok(Self(buffer))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagList(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtCompound(pub Vec<NbtNamedTag>);

impl NbtType for NbtCompound {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self>
    where
        Self: Sized,
    {
        let mut buffer = Vec::new();
        loop {
            let Some(t) = NbtNamedTag::read(r)? else {
                break;
            };

            buffer.push(t);
        }

        Ok(Self(buffer))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagCompound(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtIntArray(pub Vec<NbtInt>);

impl NbtType for NbtIntArray {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let length = NbtInt::read(r)?;
        let length = usize::try_from(length.0)?;

        let mut buffer = Vec::with_capacity(length * size_of::<NbtInt>());
        for _ in 0..length {
            buffer.push(NbtInt::read(r)?);
        }

        Ok(Self(buffer))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagIntArray(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtLongArray(pub Vec<NbtLong>);

impl NbtType for NbtLongArray {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let length = NbtInt::read(r)?;
        let length = usize::try_from(length.0)?;

        let mut buffer = Vec::with_capacity(length * size_of::<NbtLong>());
        for _ in 0..length {
            buffer.push(NbtLong::read(r)?);
        }

        Ok(Self(buffer))
    }

    fn to_tag_type(self) -> NbtTagType {
        NbtTagType::TagLongArray(self)
    }
}
