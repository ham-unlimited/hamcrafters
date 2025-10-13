use std::io::Read;

use crate::{
    NbtResult,
    nbt_types::{NbtString, NbtType},
    tag_type::NbtTagType,
};

#[derive(Debug, Clone)]
pub struct NbtNamedTag {
    pub name: NbtString,
    pub payload: NbtTagType,
}

impl NbtNamedTag {
    /// Reads a [NbtNamedTag] from the provided [r], if the byte_tag is TAG_End returns None.
    pub fn read<R: Read>(r: &mut R) -> NbtResult<Option<Self>> {
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
