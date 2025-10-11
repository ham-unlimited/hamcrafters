use serde::Serialize;

/// A Json String in the minecraft protocol.
/// [T] is the contained Json object
/// MAX_SIZE is the max allowed size of the string
#[derive(Debug, Clone)]
pub struct JsonString<T, const MAX_SIZE: usize> {
    inner: T,
}

impl<T: Serialize, const MAX_SIZE: usize> Serialize for JsonString<T, MAX_SIZE> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
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
