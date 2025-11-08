use std::fmt::Display;

use crate::{nbt_named_tag::NbtNamedTag, tag_type::NbtTagType};

// TODO: Implement From<&Snbt> for NbtNamedTag

/// SNBT type (Stringified Named Binary Tag)
#[derive(PartialEq, Debug, Clone)]
pub struct Snbt(String);

impl From<&NbtNamedTag> for Snbt {
    fn from(value: &NbtNamedTag) -> Self {
        let name = &value.name.0;

        let name = if name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "_-+.".contains(c))
        {
            name
        } else {
            &format!(r#""{name}""#)
        };
        let inner: Snbt = (&value.payload).into();
        Self(format!("{}:{}", name, inner.0))
    }
}

impl Display for Snbt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&NbtTagType> for Snbt {
    fn from(value: &NbtTagType) -> Self {
        match value {
            NbtTagType::TagEnd => Self("".to_string()),
            NbtTagType::TagByte(nbt_byte) => Self(format!("{}b", nbt_byte.0)),
            NbtTagType::TagShort(nbt_short) => Self(format!("{}s", nbt_short.0)),
            NbtTagType::TagInt(nbt_int) => Self(format!("{}", nbt_int.0)),
            NbtTagType::TagLong(nbt_long) => Self(format!("{}l", nbt_long.0)),
            NbtTagType::TagFloat(nbt_float) => Self(format!("{}f", nbt_float.0)),
            NbtTagType::TagDouble(nbt_double) => Self(format!("{}d", nbt_double.0)),
            NbtTagType::TagByteArray(nbt_byte_array) => {
                let content = nbt_byte_array
                    .0
                    .iter()
                    .map(|b| format!("{}b", b.0))
                    .collect::<Vec<String>>();
                let content = content.join(",");
                Self(format!("[B;{content}]"))
            }
            NbtTagType::TagString(nbt_string) => Self(format!(r#""{}""#, nbt_string.0)),
            NbtTagType::TagList(nbt_list) => {
                let list = nbt_list
                    .0
                    .iter()
                    .map(|v| Snbt::from(v).0)
                    .collect::<Vec<String>>();
                let list = list.join(",");

                Self(format!("[{list}]"))
            }
            NbtTagType::TagCompound(nbt_compound) => {
                let members = nbt_compound
                    .0
                    .iter()
                    .map(|c| Snbt::from(c).0)
                    .collect::<Vec<String>>();
                let members = members.join(",");
                Self(format!("{{{members}}}"))
            }
            NbtTagType::TagIntArray(nbt_int_array) => {
                let content = nbt_int_array
                    .0
                    .iter()
                    .map(|i| i.0.to_string())
                    .collect::<Vec<String>>();
                let content = content.join(",");
                Self(format!("[I;{content}]"))
            }
            NbtTagType::TagLongArray(nbt_long_array) => {
                let content = nbt_long_array
                    .0
                    .iter()
                    .map(|b| format!("{}l", b.0))
                    .collect::<Vec<String>>();
                let content = content.join(",");
                Self(format!("[L;{content}]"))
            }
        }
    }
}
