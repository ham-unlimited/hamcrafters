use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Clientbound Recipe Book Settings packet during play phase.
#[derive(Debug, Deserialize)]
#[mc_packet(0x4A)]
pub struct RecipeBookSettings {
    /// If true, then the crafting recipe book will be open when the player opens its inventory.
    pub crafting_book_open: bool,
    /// If true, then the filtering option is active when the player opens its inventory.
    pub crafting_book_filter_active: bool,
    /// If true, then the smelting recipe book will be open when the player opens its inventory.
    pub smelting_recipe_book_open: bool,
    /// If true, then the filtering option is active when the player opens its inventory.
    pub smelting_recipe_book_filter_active: bool,
    /// If true, then the blast furnace recipe book will be open when the player opens its inventory.
    pub blasting_recipe_book_open: bool,
    /// If true, then the filtering option is active when the player opens its inventory.
    pub blasting_recipe_book_filter_active: bool,
    /// If true, then the smoker recipe book will be open when the player opens its inventory.
    pub smoker_recipe_book_open: bool,
    /// If true, then the filtering option is active when the player opens its inventory.
    pub smoker_recipe_book_filter_active: bool,
}
