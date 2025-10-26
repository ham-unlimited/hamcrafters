use std::{fmt::Display, io::Read};

use crate::{NbtResult, error::NbtError, nbt_named_tag::NbtNamedTag, tag_type::NbtTagType};

/// Trait for all NbtTypes.
pub trait NbtType {
    /// Parse the implementing type from the provided [Read].
    fn read<R: Read>(r: &mut R) -> NbtResult<Self>
    where
        Self: Sized;

    // /// Return this type wrapped in NbtTagType.
    // fn to_tag_type(self) -> NbtTagType;
}

/// A signle byte in NBT format.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtByte(pub i8);

impl NbtType for NbtByte {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 1];
        r.read_exact(&mut bs)?;
        Ok(Self(i8::from_be_bytes(bs)))
    }

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagByte(self)
    // }
}

/// Two-byte integer in NBT format.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtShort(pub i16);

impl NbtType for NbtShort {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 2];
        r.read_exact(&mut bs)?;
        Ok(Self(i16::from_be_bytes(bs)))
    }

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagShort(self)
    // }
}

/// A four-byte integer in NBT format.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtInt(pub i32);

impl NbtType for NbtInt {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 4];
        r.read_exact(&mut bs)?;
        Ok(Self(i32::from_be_bytes(bs)))
    }

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagInt(self)
    // }
}

/// A eight-byte integer in NBT format.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtLong(pub i64);

impl NbtType for NbtLong {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 8];
        r.read_exact(&mut bs)?;
        Ok(Self(i64::from_be_bytes(bs)))
    }

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagLong(self)
    // }
}

/// A four-byte floating point number in NBT format.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtFloat(pub f32);

impl NbtType for NbtFloat {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 4];
        r.read_exact(&mut bs)?;
        Ok(Self(f32::from_be_bytes(bs)))
    }

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagFloat(self)
    // }
}

/// A eight-byte floating point number in NBT format.
#[derive(Debug, Clone, PartialEq)]
pub struct NbtDouble(pub f64);

impl NbtType for NbtDouble {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self> {
        let mut bs = [0u8; 8];
        r.read_exact(&mut bs)?;
        Ok(Self(f64::from_be_bytes(bs)))
    }

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagDouble(self)
    // }
}

/// An array containing NbtBytes.
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

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagByteArray(self)
    // }
}

/// A string in NBT format.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagString(self)
    // }
}

/// A list containing any NBT type, expected to only contain a single nbt type.
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

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagList(self)
    // }
}

/// An NBT compound, basically a JSON map in NBT format.
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

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagCompound(self)
    // }
}

/// An array of integers in NBT format.
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

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagIntArray(self)
    // }
}

/// An array of longs in NBT format.
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

    // fn to_tag_type(self) -> NbtTagType {
    //     NbtTagType::TagLongArray(self)
    // }
}

impl Display for NbtTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", match self {
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
        }))
    }
}
