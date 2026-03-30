use nbt::{nbt_types::NbtCompound, tag_type::NbtTagType};
use serde::de::Error;

/// A minecraft Text Component
#[derive(Debug)]
pub enum TextComponent {
    /// A pure string text component.
    Literal(String),
    /// A compound text component.
    Compound(NbtCompound),
}

impl<'de> serde::Deserialize<'de> for TextComponent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let nbt = NbtTagType::deserialize(deserializer)
            .map_err(|err| Error::custom(format!("Failed to deserialize TextComponent: {err}")))?;
        match nbt {
            NbtTagType::TagString(s) => Ok(Self::Literal(s.0)),
            NbtTagType::TagCompound(c) => Ok(Self::Compound(c)),
            other => Err(Error::custom(format!(
                "Invalid NBT type for TextComponent: expected String or Compound, got {other:?}"
            ))),
        }
    }
}
