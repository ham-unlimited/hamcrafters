use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Clientbound entity event packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x22)]
pub struct EntityEvent {
    /// The ID of the entity.
    pub entity_id: i32,
    /// The status of the entity.
    // TODO Is really an enum but the variants depend on the entity in question.
    pub entity_status: u8,
}
