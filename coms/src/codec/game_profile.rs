use serde::Deserialize;
use uuid::Uuid;

use crate::codec::{prefixed_array::PrefixedArray, prefixed_optional::PrefixedOptional};

#[derive(Debug, Deserialize)]
pub struct GameProfile {
    uuid: Uuid,
    username: String,
    properties: PrefixedArray<Property>,
}

#[derive(Debug, Deserialize)]
pub struct Property {
    name: String,
    value: String,
    signature: PrefixedOptional<String>,
}
