//! Generic NBT deserialized value, intended to function similar to serde_json's Value type.

use std::{collections::HashMap, fmt::Display};

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
        write!(f, "{}", match self {
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
                    .map(|(a, b)| format!("\"{a}\": {}", b.to_string()))
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
        })
    }
}
