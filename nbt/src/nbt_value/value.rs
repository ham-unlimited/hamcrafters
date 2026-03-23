//! Generic NBT deserialized value, intended to function similar to serde_json's Value type.

use std::{collections::HashMap, fmt::Display};

use crate::{nbt_named_tag::NbtNamedTag, tag_type::NbtTagType};

/// Generic NBT value.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum NbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NbtValue>),
    Compound(HashMap<String, NbtValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Display for NbtValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NbtValue::Byte(b) => b.to_string(),
                NbtValue::Short(s) => s.to_string(),
                NbtValue::Int(i) => i.to_string(),
                NbtValue::Long(l) => l.to_string(),
                NbtValue::Float(f) => f.to_string(),
                NbtValue::Double(d) => d.to_string(),
                NbtValue::ByteArray(bytes) => {
                    let bs = bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>();
                    format!("[{}]", bs.join(", "))
                }
                NbtValue::String(s) => format!("\"{s}\""),
                NbtValue::List(nbt_values) => {
                    let vs = nbt_values.iter().map(|v| v.to_string()).collect::<Vec<_>>();
                    format!("[{}]", vs.join(", "))
                }
                NbtValue::Compound(hash_map) => {
                    let vs = hash_map
                        .iter()
                        .map(|(a, b)| format!("\"{a}\": {}", b))
                        .collect::<Vec<_>>();
                    format!("{{{}}}", vs.join(", "))
                }
                NbtValue::IntArray(items) => {
                    let is = items.iter().map(|i| i.to_string()).collect::<Vec<_>>();
                    format!("[{}]", is.join(", "))
                }
                NbtValue::LongArray(items) => {
                    let ls = items.iter().map(|l| l.to_string()).collect::<Vec<_>>();
                    format!("[{}]", ls.join(", "))
                }
            }
        )
    }
}

impl From<NbtNamedTag> for NbtValue {
    fn from(value: NbtNamedTag) -> Self {
        let mut map = HashMap::new();
        if let Some(payload) = value.payload.into() {
            map.insert(value.name.0, payload);
        }
        Self::Compound(map)
    }
}

impl From<NbtTagType> for Option<NbtValue> {
    fn from(value: NbtTagType) -> Self {
        Some(match value {
            NbtTagType::TagEnd => return None,
            NbtTagType::TagByte(nbt_byte) => NbtValue::Byte(nbt_byte.0),
            NbtTagType::TagShort(nbt_short) => NbtValue::Short(nbt_short.0),
            NbtTagType::TagInt(nbt_int) => NbtValue::Int(nbt_int.0),
            NbtTagType::TagLong(nbt_long) => NbtValue::Long(nbt_long.0),
            NbtTagType::TagFloat(nbt_float) => NbtValue::Float(nbt_float.0),
            NbtTagType::TagDouble(nbt_double) => NbtValue::Double(nbt_double.0),
            NbtTagType::TagByteArray(nbt_byte_array) => {
                NbtValue::ByteArray(nbt_byte_array.0.into_iter().map(|i| i.0).collect())
            }
            NbtTagType::TagString(nbt_string) => NbtValue::String(nbt_string.0),
            NbtTagType::TagList(nbt_list) => NbtValue::List(
                nbt_list
                    .0
                    .into_iter()
                    .map(|l| l.into())
                    .filter_map(|l| l)
                    .collect(),
            ),
            NbtTagType::TagCompound(nbt_compound) => NbtValue::Compound(
                nbt_compound
                    .0
                    .into_iter()
                    .filter_map(|n| {
                        let o: Option<NbtValue> = n.payload.into();
                        o.map(|value| (n.name.0, value))
                    })
                    .collect(),
            ),
            NbtTagType::TagIntArray(nbt_int_array) => {
                NbtValue::IntArray(nbt_int_array.0.into_iter().map(|i| i.0).collect())
            }
            NbtTagType::TagLongArray(nbt_long_array) => {
                NbtValue::LongArray(nbt_long_array.0.into_iter().map(|l| l.0).collect())
            }
        })
    }
}
