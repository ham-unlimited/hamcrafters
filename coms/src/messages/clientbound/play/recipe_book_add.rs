use crate::{
    McPacket,
    codec::{prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::id_set::IdSet,
};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Clientbound Recipe Book Add packet during play phase.
#[derive(Debug, Deserialize)]
#[mc_packet(0x48)]
pub struct RecipeBookAdd {
    /// The recipes to add.
    pub recipes: PrefixedArray<()>,
    /// Replace or Add to known recipes.
    pub replace: bool,
}
