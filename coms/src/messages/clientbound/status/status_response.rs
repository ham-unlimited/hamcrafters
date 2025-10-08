use crate::McPacket;
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[mc_packet(0x00)]
pub struct StatusResponse {
    version: StatusResponseVersion,
    players: Option<StatusResponsePlayers>,
    // Unclear, server seems to give String but spec says otherwise :shrug:
    description: Option<String>,
    favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforces_secure_chat: Option<bool>,
}

impl StatusResponse {
    pub fn new() -> Self {
        Self {
            version: StatusResponseVersion {
                name: "1.28.9".to_string(),
                protocol: 773,
            },
            players: Some(StatusResponsePlayers {
                max: 20,
                online: 0,
                sample: None,
            }),
            description: Some("TEST Server".to_string()),
            favicon: None,
            enforces_secure_chat: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponseVersion {
    name: String,
    protocol: u32,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponsePlayers {
    max: u32,
    online: u32,
    sample: Option<Vec<StatusResponsePlayersSample>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponsePlayersSample {
    name: String,
    id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponseDescription {
    text: String,
}
