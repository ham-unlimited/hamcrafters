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
        let json_string = serde_json::to_string(&self.inner).map_err(serde::ser::Error::custom)?;

        if json_string.len() > MAX_SIZE {
            return Err(serde::ser::Error::custom(format!(
                "Invalid length for JsonString, expected max_length: {MAX_SIZE}, got length: {}",
                json_string.len()
            )));
        }

        json_string.serialize(serializer)
    }
}

// TODO: Do we want this? We don't validate that Default::default() respects MAX_SIZE and validating it would require a panic since default must result Self (not Result).
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

        if s.len() > MAX_SIZE {
            return Err(serde::de::Error::custom(format!(
                "JsonString longer than max allowed value for this field, max {MAX_SIZE}, got {}",
                s.len()
            )));
        }

        let t: T = serde_json::from_str(&s).map_err(serde::de::Error::custom)?;

        Ok(Self { inner: t })
    }
}
