use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::{id_set::IdSet, slot_display::SlotDisplay},
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};

/// Client-bound update time packet, used to update the world time and the day time.
/// Time is based on ticks, where 20 ticks happen every second. There are 24000 ticks in a day, making Minecraft days exactly 20 minutes long.
/// The time of day is based on the timestamp modulo 24000. 0 is sunrise, 6000 is noon, 12000 is sunset, and 18000 is midnight.
/// The default SMP server increments the time by 20 every second.
#[derive(Debug, Deserialize)]
#[mc_packet(0x6F)]
pub struct UpdateTime {
    /// The age of the world, in ticks. Not changed by any server commands.
    pub world_age: i64,
    /// The world (or region) time, in ticks.
    pub time_of_day: i64,
    /// If true, the client should automatically advance the time of day according to its ticking rate.
    pub time_of_day_increasing: bool,
}
