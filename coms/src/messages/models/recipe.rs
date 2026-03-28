use serde::Deserialize;

use crate::{
    codec::{prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::id_set::IdSet,
};

/// A crafting recipe
#[derive(Debug, Deserialize)]
pub struct Recipe {
    pub recipe_id: VarInt,
    pub display: RecipeDisplay,
    pub group_id: VarInt,
    pub category_id: VarInt,
    pub ingredients: Option<PrefixedArray<IdSet>>,
    pub flags: RecipeFlags,
}

/// Flags for a recipe.
#[derive(Debug, Deserialize)]
pub struct RecipeFlags {}
