use std::fmt;

use nbt::{
    nbt_named_tag::NbtNamedTag,
    nbt_types::{NbtCompound, NbtString},
    tag_type::NbtTagType,
};
use serde::de::{self, Visitor};

/// A minecraft Text Component
#[derive(Debug)]
pub enum TextComponent {
    /// A pure string text component.
    Literal(NbtString),
    /// A compound text component.
    Compound(NbtCompound),
}

struct TextComponentVisitor;

impl<'de> Visitor<'de> for TextComponentVisitor {
    type Value = TextComponent;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an NBT string or compound tag")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(TextComponent::Literal(NbtString(v.to_owned())))
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(TextComponent::Literal(NbtString(v)))
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut entries = Vec::new();
        while let Some((k, v)) = map.next_entry::<String, NbtTagType>()? {
            entries.push(NbtNamedTag {
                name: NbtString(k),
                payload: v,
            });
        }
        Ok(TextComponent::Compound(NbtCompound(entries)))
    }
}

impl<'de> serde::Deserialize<'de> for TextComponent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(TextComponentVisitor)
    }
}
