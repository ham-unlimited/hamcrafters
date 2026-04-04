use serde::{Deserialize, de::Error};

use crate::{
    codec::{prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::{id_set::IdSet, slot_display::SlotDisplay},
};

/// A crafting recipe
#[derive(Debug, Deserialize)]
pub struct Recipe {
    /// The ID of the recipe.
    pub recipe_id: VarInt,
    /// The display information for the recipe.
    pub display: RecipeDisplay,
    /// The group ID for the recipe (used for recipe book grouping).
    pub group_id: VarInt,
    /// The category ID for the recipe, ID in the minecraft:recipe_book_category registry.
    pub category_id: VarInt,
    /// The ingredients for the recipe. IDs in the minecraft:item registry.
    pub ingredients: Option<PrefixedArray<IdSet>>,
    /// The flags for the recipe.
    pub flags: RecipeFlags,
}

/// Flags for a recipe.
#[derive(Debug)]
pub struct RecipeFlags {
    /// Whether to show a notification when the recipe is unlocked.
    pub show_notification: bool,
    /// Whether to highlight the recipe as new.
    pub highlight_as_new: bool,
}

impl<'de> Deserialize<'de> for RecipeFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let flags = u8::deserialize(deserializer)?;
        Ok(Self {
            show_notification: flags & 0x1 != 0,
            highlight_as_new: flags & 0x2 != 0,
        })
    }
}

/// Display information for a recipe.
#[derive(Debug)]
pub enum RecipeDisplay {
    /// A shapeless crafting recipe.
    CraftingShapeless(CraftingShapelessDisplay),
    /// A shaped crafting recipe.
    CraftingShaped(CraftingShapedDisplay),
    /// A furnace recipe.
    Furnace(Furnace),
    /// A stonecutter recipe.
    StoneCutter(StoneCutterDisplay),
    /// A smithing recipe.
    Smithing(SmithingDisplay),
}

impl<'de> Deserialize<'de> for RecipeDisplay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RecipeDisplayVisitor;

        impl<'de> serde::de::Visitor<'de> for RecipeDisplayVisitor {
            type Value = RecipeDisplay;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a recipe display")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let id: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| Error::custom("missing recipe display type ID"))?;

                match id.0 {
                    0 => {
                        let crafting_shapeless = seq
                            .next_element()
                            .map_err(|e| {
                                Error::custom(format!(
                                    "failed to parse shapeless crafting display data: {e}"
                                ))
                            })?
                            .ok_or_else(|| {
                                Error::custom("missing crafting shapeless display data")
                            })?;
                        Ok(RecipeDisplay::CraftingShapeless(crafting_shapeless))
                    }
                    1 => {
                        let crafting_shaped = seq
                            .next_element()
                            .map_err(|e| {
                                Error::custom(format!(
                                    "failed to parse crafting shaped display data: {e}"
                                ))
                            })?
                            .ok_or_else(|| Error::custom("missing crafting shaped display data"))?;
                        Ok(RecipeDisplay::CraftingShaped(crafting_shaped))
                    }
                    2 => {
                        let furnace = seq
                            .next_element()
                            .map_err(|e| {
                                Error::custom(format!("failed to parse furnace display data: {e}"))
                            })?
                            .ok_or_else(|| Error::custom("missing furnace display data"))?;
                        Ok(RecipeDisplay::Furnace(furnace))
                    }
                    3 => {
                        let stonecutter = seq
                            .next_element()
                            .map_err(|e| {
                                Error::custom(format!(
                                    "failed to parse stonecutter display data: {e}"
                                ))
                            })?
                            .ok_or_else(|| Error::custom("missing stonecutter display data"))?;
                        Ok(RecipeDisplay::StoneCutter(stonecutter))
                    }
                    4 => {
                        let smithing = seq
                            .next_element()
                            .map_err(|e| {
                                Error::custom(format!("failed to parse smithing display data: {e}"))
                            })?
                            .ok_or_else(|| Error::custom("missing smithing display data"))?;
                        Ok(RecipeDisplay::Smithing(smithing))
                    }
                    id => Err(Error::custom(format!(
                        "unknown recipe display type ID: {id}"
                    ))),
                }
            }
        }

        deserializer.deserialize_seq(RecipeDisplayVisitor)
    }
}

/// Information regarding a shapeless crafting recipe.
#[derive(Debug, Deserialize)]
pub struct CraftingShapelessDisplay {
    /// The ingredients for the recipe, in the protocol this is an "ingredients_count" VarInt field followed by an array but that is identical to a PrefixedArray.
    pub ingredients: PrefixedArray<SlotDisplay>,
    /// The result of the recipe.
    pub result: SlotDisplay,
    /// The crafting station for the recipe.
    pub crafting_station: SlotDisplay,
}

/// Information regarding a shaped crafting recipe.
#[derive(Debug, Deserialize)]
pub struct CraftingShapedDisplay {
    /// The width of the crafting grid.
    pub width: VarInt,
    /// The height of the crafting grid.
    pub height: VarInt,
    /// The ingredients for the recipe, in the protocol this is an "ingredients_count" VarInt field followed by an array but that is identical to a PrefixedArray (length should always be width * height).
    pub ingredients: PrefixedArray<SlotDisplay>,
    /// The result of the recipe.
    pub result: SlotDisplay,
    /// The crafting station for the recipe.
    pub crafting_station: SlotDisplay,
}

/// A furnace recipe.
#[derive(Debug, Deserialize)]
pub struct Furnace {
    /// The ingredient for the recipe.
    pub ingredient: SlotDisplay,
    /// The fuel for the recipe.
    pub fuel: SlotDisplay,
    /// The result of the recipe.
    pub result: SlotDisplay,
    /// The crafting station for the recipe.
    pub crafting_station: SlotDisplay,
    /// The cooking time for the recipe.
    pub cooking_time: VarInt,
    /// The experience gained from the recipe.
    pub experience: f32,
}

/// A stonecutter recipe.
#[derive(Debug, Deserialize)]
pub struct StoneCutterDisplay {
    /// The ingredient for the recipe.
    pub ingredient: SlotDisplay,
    /// The result of the recipe.
    pub result: SlotDisplay,
    /// The crafting station for the recipe.
    pub crafting_station: SlotDisplay,
}

/// A smithing recipe.
#[derive(Debug, Deserialize)]
pub struct SmithingDisplay {
    /// The template item for the recipe.
    pub template: SlotDisplay,
    /// The base item for the recipe.
    pub base: SlotDisplay,
    /// The addition item for the recipe.
    pub addition: SlotDisplay,
    /// The result of the recipe.
    pub result: SlotDisplay,
    /// The crafting station for the recipe.
    pub crafting_station: SlotDisplay,
}
