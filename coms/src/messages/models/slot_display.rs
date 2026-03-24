use std::fmt;

use serde::{
    Serialize,
    de::{self, SeqAccess, Visitor},
    ser::SerializeStruct,
};

use crate::{
    codec::{identifier::Identifier, var_int::VarInt},
    messages::models::slot::Slot,
};

/// A minecraft slot display. Encoded as a VarInt type ID followed by type-specific data.
#[derive(Debug)]
pub enum SlotDisplay {
    /// An empty slot display. (0)
    Empty,
    /// Any fuel slot display. (1)
    AnyFuel,
    /// An item slot display, references an ID in the "minecraft:item" registry. (2)
    Item(VarInt),
    /// Stack of items. (3)
    ItemStack(Slot),
    /// Tag in the minecraft:item registry (not prefixed by "#"). (4)
    Tag(Identifier),
    /// Smithing trim. (5)
    SmithingTrim {
        /// Base item.
        base: Box<SlotDisplay>,
        /// The material to apply.
        material: Box<SlotDisplay>,
        /// The pattern to use, item ID in the trim_pattern registry.
        pattern: VarInt,
    },
    /// Item with a remainder left behind after use. (6)
    WithRemainder {
        /// The ingredient.
        ingredient: Box<SlotDisplay>,
        /// The remainder.
        remainder: Box<SlotDisplay>,
    },
    /// A composite of multiple slot displays. (7)
    Composite(Vec<SlotDisplay>),
}

impl SlotDisplay {
    const ID_EMPTY: i32 = 0;
    const ID_ANY_FUEL: i32 = 1;
    const ID_ITEM: i32 = 2;
    const ID_ITEM_STACK: i32 = 3;
    const ID_TAG: i32 = 4;
    const ID_SMITHING_TRIM: i32 = 5;
    const ID_WITH_REMAINDER: i32 = 6;
    const ID_COMPOSITE: i32 = 7;

    /// Returns the protocol type ID for this slot display variant.
    pub fn type_id(&self) -> VarInt {
        VarInt(match self {
            Self::Empty => Self::ID_EMPTY,
            Self::AnyFuel => Self::ID_ANY_FUEL,
            Self::Item(_) => Self::ID_ITEM,
            Self::ItemStack(_) => Self::ID_ITEM_STACK,
            Self::Tag(_) => Self::ID_TAG,
            Self::SmithingTrim { .. } => Self::ID_SMITHING_TRIM,
            Self::WithRemainder { .. } => Self::ID_WITH_REMAINDER,
            Self::Composite(_) => Self::ID_COMPOSITE,
        })
    }

    /// Returns the number of fields serialized for this variant (including the type ID field).
    fn field_count(&self) -> usize {
        match self {
            Self::Empty | Self::AnyFuel => 1,
            Self::Item(_) | Self::ItemStack(_) | Self::Tag(_) => 2,
            Self::WithRemainder { .. } => 3,
            Self::SmithingTrim { .. } => 4,
            // type_id + count + N options
            Self::Composite(options) => 2 + options.len(),
        }
    }
}

struct SlotDisplayVisitor;

impl<'de> Visitor<'de> for SlotDisplayVisitor {
    type Value = SlotDisplay;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "a slot display (VarInt type ID followed by type-specific data)"
        )
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let type_id: VarInt = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("missing slot display type ID"))?;

        match type_id.0 {
            SlotDisplay::ID_EMPTY => Ok(SlotDisplay::Empty),
            SlotDisplay::ID_ANY_FUEL => Ok(SlotDisplay::AnyFuel),
            SlotDisplay::ID_ITEM => {
                let item: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing item type for minecraft:item"))?;
                Ok(SlotDisplay::Item(item))
            }
            SlotDisplay::ID_ITEM_STACK => {
                let slot: Slot = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing slot for minecraft:item_stack"))?;
                Ok(SlotDisplay::ItemStack(slot))
            }
            SlotDisplay::ID_TAG => {
                let tag: Identifier = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing tag for minecraft:tag"))?;
                Ok(SlotDisplay::Tag(tag))
            }
            SlotDisplay::ID_SMITHING_TRIM => {
                let base: SlotDisplay = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::custom("missing base for minecraft:smithing_trim"))?;
                let material: SlotDisplay = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom("missing material for minecraft:smithing_trim")
                })?;
                let pattern: VarInt = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom("missing pattern for minecraft:smithing_trim")
                })?;
                Ok(SlotDisplay::SmithingTrim {
                    base: Box::new(base),
                    material: Box::new(material),
                    pattern,
                })
            }
            SlotDisplay::ID_WITH_REMAINDER => {
                let ingredient: SlotDisplay = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom("missing ingredient for minecraft:with_remainder")
                })?;
                let remainder: SlotDisplay = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom("missing remainder for minecraft:with_remainder")
                })?;
                Ok(SlotDisplay::WithRemainder {
                    ingredient: Box::new(ingredient),
                    remainder: Box::new(remainder),
                })
            }
            SlotDisplay::ID_COMPOSITE => {
                let count: VarInt = seq.next_element()?.ok_or_else(|| {
                    de::Error::custom("missing option count for minecraft:composite")
                })?;
                let mut options = Vec::with_capacity(count.0 as usize);
                for _ in 0..count.0 {
                    let option: SlotDisplay = seq.next_element()?.ok_or_else(|| {
                        de::Error::custom("missing option for minecraft:composite")
                    })?;
                    options.push(option);
                }
                Ok(SlotDisplay::Composite(options))
            }
            id => Err(de::Error::custom(format!(
                "unknown slot display type ID: {id}"
            ))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for SlotDisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_seq(SlotDisplayVisitor)
    }
}

impl Serialize for SlotDisplay {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_struct writes fields inline without a length prefix, matching
        // the wire format: VarInt type ID followed immediately by the variant's fields.
        let mut s = serializer.serialize_struct("SlotDisplay", self.field_count())?;
        s.serialize_field("type_id", &self.type_id())?;
        match self {
            SlotDisplay::Empty | SlotDisplay::AnyFuel => {}
            SlotDisplay::Item(item) => {
                s.serialize_field("item", item)?;
            }
            SlotDisplay::ItemStack(slot) => {
                s.serialize_field("slot", slot)?;
            }
            SlotDisplay::Tag(tag) => {
                s.serialize_field("tag", tag)?;
            }
            SlotDisplay::SmithingTrim {
                base,
                material,
                pattern,
            } => {
                s.serialize_field("base", base)?;
                s.serialize_field("material", material)?;
                s.serialize_field("pattern", pattern)?;
            }
            SlotDisplay::WithRemainder {
                ingredient,
                remainder,
            } => {
                s.serialize_field("ingredient", ingredient)?;
                s.serialize_field("remainder", remainder)?;
            }
            SlotDisplay::Composite(options) => {
                s.serialize_field("count", &VarInt(options.len() as i32))?;
                for option in options {
                    s.serialize_field("option", option)?;
                }
            }
        }
        s.end()
    }
}
