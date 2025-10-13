use std::{
    fs::File,
    io::{self, Cursor, Read, Write},
    num::TryFromIntError,
    path::Path,
    string::FromUtf8Error,
};

use flate2::read::GzDecoder;

#[derive(Debug, thiserror::Error)]
pub enum NbtError {
    #[error("IO Error `{0}`")]
    IoError(#[from] io::Error),
    #[error("Encountered invalid nbt tag {0}")]
    InvalidNbtTag(u8),
    #[error("Malformed NBT, err: {0}")]
    MalformedNbt(String),
    #[error("String contained invalid utf8, err: {0}")]
    InvalidUtf8(#[from] FromUtf8Error),
    #[error("Provided length was invalid, err: {0}")]
    InvalidLength(#[from] TryFromIntError),
}

pub type NbtResult<T> = Result<T, NbtError>;

pub fn read_nbt_file(path: &Path) -> NbtResult<()> {
    let file = File::open(path)?;
    let mut decoder = GzDecoder::new(file);

    let mut file_content = Vec::new();
    decoder.read_to_end(&mut file_content)?;

    let mut file_content = Cursor::new(file_content);

    let first_tag = NbtNamedTag::read(&mut file_content)?;
    println!("Minecraft NBT file: {first_tag:#?}");

    todo!();

    Ok(())
}

#[derive(Debug, Clone)]
struct NbtNamedTag {
    pub name: NbtString,
    pub payload: NbtTagType,
}

impl NbtNamedTag {
    /// Reads a [NbtNamedTag] from the provided [r], if the byte_tag is TAG_End returns None.
    fn read<R: Read>(r: &mut R) -> NbtResult<Option<Self>> {
        let mut tag_type = [0u8; 1];
        r.read_exact(&mut tag_type)?;

        if tag_type[0] == NbtTagType::TagEnd.get_tag_id() {
            return Ok(None);
        }

        let name = NbtString::read(r)?;

        let payload = NbtTagType::read(tag_type[0], r)?;

        Ok(Some(Self { name, payload }))
    }
}

trait NbtType {
    fn read<R: Read>(r: &mut R) -> NbtResult<Self>
    where
        Self: Sized;

    fn to_tag_type(self) -> NbtTagType;
}

#[derive(Debug, Clone)]
struct NbtByte(i8);

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

#[derive(Debug, Clone)]
struct NbtShort(i16);

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

#[derive(Debug, Clone)]
struct NbtInt(i32);

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

#[derive(Debug, Clone)]
struct NbtLong(i64);

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

#[derive(Debug, Clone)]
struct NbtFloat(f32);

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

#[derive(Debug, Clone)]
struct NbtDouble(f64);

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
#[derive(Debug, Clone)]
struct NbtByteArray(Vec<NbtByte>);

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

#[derive(Debug, Clone)]
struct NbtString(String);

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

#[derive(Debug, Clone)]
struct NbtList(Vec<NbtTagType>);

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

#[derive(Debug, Clone)]
struct NbtCompound(Vec<NbtNamedTag>);

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

#[derive(Debug, Clone)]
struct NbtIntArray(Vec<NbtInt>);

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

#[derive(Debug, Clone)]
struct NbtLongArray(Vec<NbtLong>);

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

#[derive(Debug, Clone)]
enum NbtTagType {
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
    fn read<R: Read>(tag_id: u8, r: &mut R) -> NbtResult<Self> {
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

    fn get_tag_id(&self) -> u8 {
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_level_dat() {
        fs::read_dir("../test-data")
            .unwrap()
            .into_iter()
            .for_each(|d| {
                println!("{:?}", d.unwrap());
            });
        read_nbt_file(Path::new("../test-data/level.dat")).expect("Expect to read file correctly");
    }
}
