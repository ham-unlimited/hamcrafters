use crate::{
    McPacket,
    codec::{prefixed_array::PrefixedArray, prefixed_optional::PrefixedOptional},
};
use mc_packet_macros::mc_packet;
use serde::Deserialize;
use uuid::Uuid;

/// Login success message, returns the game profile of the mc player
#[derive(Debug, Deserialize)]
#[mc_packet(0x02)]
pub struct LoginSuccess {
    profile: GameProfile,
}

/// A minecraft game profile
#[derive(Debug, Deserialize)]
pub struct GameProfile {
    uuid: Uuid,
    username: String,
    properties: PrefixedArray<Property>,
}

#[derive(Debug, Deserialize)]
struct Property {
    name: String,
    value: String,
    signature: PrefixedOptional<String>,
}
