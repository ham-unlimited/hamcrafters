use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// A Json String in the minecraft protocol.
/// [T] is the contained Json object
/// MAX_SIZE is the max allowed size of the string
#[derive(Debug, Clone)]
pub struct JsonString<T, const MAX_SIZE: usize> {
    inner: T,
}

impl<T, const MAX_SIZE: usize> JsonString<T, MAX_SIZE> {
    /// Converts this into the inner value.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: Serialize, const MAX_SIZE: usize> Serialize for JsonString<T, MAX_SIZE> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Validate length using MAX_SIZE.
        let json_string = serde_json::to_string(&self.inner).map_err(serde::ser::Error::custom)?;

        json_string.serialize(serializer)
    }
}

impl<T: Default, const MAX_SIZE: usize> Default for JsonString<T, MAX_SIZE> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<'de, T: DeserializeOwned, const MAX_SIZE: usize> Deserialize<'de> for JsonString<T, MAX_SIZE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: Maybe a bit of unnecessary allocations here, also only Owned supported atm.
        let s = String::deserialize(deserializer)?;

        // TODO: Validate size.
        let t: T = serde_json::from_str(&s).map_err(serde::de::Error::custom)?;

        Ok(Self { inner: t })
    }
}
