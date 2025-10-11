use crate::{McPacket, codec::json_string::JsonString};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A response to a Minecraft status request.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default)]
#[mc_packet(0x00)]
pub struct StatusResponse(JsonString<ServerStatus, 27512>);

/// Server status inner response.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatus {
    version: ServerStatusVersion,
    players: Option<ServerStatusPlayers>,
    // Unclear, server seems to give String but spec says otherwise :shrug:
    description: Option<String>,
    favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforces_secure_chat: Option<bool>,
}

impl Default for ServerStatus {
    fn default() -> Self {
        Self {
            version: ServerStatusVersion {
                name: "1.28.10".to_string(),
                protocol: 773,
            },
            players: Some(ServerStatusPlayers {
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

/// The Minecraft version of this server implementation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerStatusVersion {
    name: String,
    protocol: u32,
}

/// The status of players online on this server right now.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerStatusPlayers {
    max: u32,
    online: u32,
    sample: Option<Vec<ServerStatusPlayersSample>>,
}

/// Information about a currently logged in player on this server.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerStatusPlayersSample {
    name: String,
    id: Uuid,
}

/// A description (called MOTD in vanilla servers) of this server.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerStatusDescription {
    text: String,
}
