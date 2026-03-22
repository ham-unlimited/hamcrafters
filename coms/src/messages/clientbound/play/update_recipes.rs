use std::path::Prefix;

use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::{id_set::IdSet, slot_display::SlotDisplay},
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound update recipes packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x83)]
pub struct UpdateRecipes {
    /// The list of property sets to update.
    pub property_sets: PrefixedArray<PropertySet>,
    /// The list of stone cutter crafting recipes to update.
    pub stone_cutter_recipes: PrefixedArray<StonecutterRecipe>,
}

/// A property set to update in the update recipes packet.
#[derive(Debug, Deserialize)]
pub struct PropertySet {
    /// The ID of the property set to update.
    pub property_set_id: Identifier,
    /// The list of item IDs in the property set. These are IDs in the minecraft:item registry.
    pub items: PrefixedArray<VarInt>,
}

/// A stonecutter recipe to update in the update recipes packet.
#[derive(Debug, Deserialize)]
pub struct StonecutterRecipe {
    /// The ingredients of the stonecutter recipe.
    pub ingredients: IdSet,
    /// The slot display of the stonecutter recipe.
    pub slot_display: SlotDisplay,
}
