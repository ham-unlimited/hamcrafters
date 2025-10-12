use crate::{McPacket, codec::game_profile::GameProfile};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[mc_packet(0x02)]
pub struct LoginSuccess {
    pub profile: GameProfile,
}
