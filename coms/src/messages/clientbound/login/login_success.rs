use crate::{McPacket, codec::game_profile::GameProfile};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Login success message, returns the game profile of the mc player
#[derive(Debug, Deserialize)]
#[mc_packet(0x02)]
pub struct LoginSuccess {
    profile: GameProfile,
}
