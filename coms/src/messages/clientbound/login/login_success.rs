use crate::{
    McPacket,
    codec::{prefixed_array::PrefixedArray, prefixed_optional::PrefixedOptional},
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Login success message, returns the game profile of the mc player
#[derive(Debug, Deserialize, Serialize)]
#[mc_packet(0x02)]
pub struct LoginSuccess {
    pub profile: GameProfile,
}

/// A minecraft game profile
#[derive(Debug, Deserialize, Serialize)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
    pub properties: PrefixedArray<Property>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Property {
    name: String,
    value: String,
    signature: Option<String>,
}
