use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PingRequest {
    pub timestamp: i64,
}
