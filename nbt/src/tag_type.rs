use std::io::{Read, Write};

use crate::{
    NbtError, NbtResult,
    nbt_types::{
        NbtByte, NbtByteArray, NbtCompound, NbtDouble, NbtFloat, NbtInt, NbtIntArray, NbtList,
        NbtLong, NbtLongArray, NbtShort, NbtString, NbtType,
    },
};

#[derive(Debug, Clone)]
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

    fn write<W: Write>(&self, w: &mut W) -> NbtResult<()> {
        let b: u8 = self.get_tag_id();
        w.write(&[b])?;

        match self {
            NbtTagType::TagEnd => {}
            t => todo!("Writing of nbttype {t:?} not implemented"),
        }

        Ok(())
    }
}
