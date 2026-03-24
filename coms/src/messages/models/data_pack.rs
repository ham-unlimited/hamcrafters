use serde::Deserialize;

/// A datapack.
#[derive(Debug, Deserialize)]
pub struct DataPack {
    namespace: String,
    id: String,
    version: String,
}
