use crate::{
    McPacket,
    codec::{prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::slot::Slot,
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Clientbound set container content packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x12)]
pub struct SetContainerContent {
    /// The ID of window which items are being sent for. 0 for player inventory.
    /// The client ignores any packets targeting a Window ID other than the current one.
    /// However, an exception is made for the player inventory, which may be targeted at any time.
    /// (The vanilla server does not appear to utilize this special case.)
    pub window_id: VarInt,
    /// The state ID for the container update, used for synchronization.
    pub state_id: VarInt,
    /// The list of item stacks representing the contents of the container.
    pub slot_data: PrefixedArray<Slot>,
    /// The item stack currently being carried by the player's cursor, if any.
    pub carried_item: Slot,
}
