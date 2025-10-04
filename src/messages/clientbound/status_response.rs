use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponse {
    version: StatusResponseVersion,
    players: Option<StatusResponsePlayers>,
    // Unclear, server seems to give String but spec says otherwise :shrug:
    description: Option<String>,
    favicon: Option<String>,
    enforces_secure_chat: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponseVersion {
    name: String,
    protocol: u32,
}

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
