use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper type for UUIDs in minecraft protocol.
/// We need custom ser/de because Uuid by default tries to encode it as a string while the minecraft procol just encodes it as raw bytes.
#[derive(Debug, Clone)]
pub struct McUuid(Uuid);

impl McUuid {
    /// Retrieve the contained UUID value.
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for McUuid {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl Serialize for McUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let uuid = self.0.as_u128();
        serializer.serialize_u128(uuid)
    }
}

impl<'de> Deserialize<'de> for McUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let n = u128::deserialize(deserializer)?;
        let uuid = Uuid::from_u128(n);
        Ok(Self(uuid))
    }
}
