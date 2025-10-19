use serde::Deserialize;
use uuid::Uuid;

use crate::codec::{prefixed_array::PrefixedArray, prefixed_optional::PrefixedOptional};

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
