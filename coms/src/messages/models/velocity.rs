use serde::{
    Deserialize,
    de::{self, Visitor},
};

use crate::codec::var_int::VarInt;

const QUANTIZED_MASK: u64 = 32767;
const MAX_QUANTIZED_VALUE: u64 = 32766;
const MAX_QUANTIZED_VALUE_F64: f64 = MAX_QUANTIZED_VALUE as f64;
const SCALE: f64 = 2.0 / MAX_QUANTIZED_VALUE_F64;

/// A velocity in minecraft (LpVec3).
#[derive(Debug)]
pub struct Velocity {
    /// Velocity on the X axis.
    pub x: f64,
    /// Velocity on the Y axis.
    pub y: f64,
    /// Velocity on the Z axis.
    pub z: f64,
}

fn velocity_epsilon(lhs: &Velocity, rhs: &Velocity) -> f64 {
    let inferred_scale = lhs
        .x
        .abs()
        .max(lhs.y.abs())
        .max(lhs.z.abs())
        .max(rhs.x.abs())
        .max(rhs.y.abs())
        .max(rhs.z.abs())
        .ceil();
    // Rounding during pack/unpack introduces up to one half-step in normalized
    // space, i.e. 1 / 32767, then scaled by the encoded scale factor.
    inferred_scale / MAX_QUANTIZED_VALUE_F64
}

impl PartialEq for Velocity {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = velocity_epsilon(self, other);
        (self.x - other.x).abs() <= epsilon
            && (self.y - other.y).abs() <= epsilon
            && (self.z - other.z).abs() <= epsilon
    }
}

impl Velocity {
    /// Create a new velocity with the given components.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Velocity { x, y, z }
    }

    /// Create a new velocity with all components set to zero.
    pub fn zero() -> Self {
        Velocity {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

struct VelocityVisitor;

fn unpack(packed: u64) -> f64 {
    let v = (packed & QUANTIZED_MASK) as f64;
    let v = v.min(MAX_QUANTIZED_VALUE_F64);
    v * SCALE - 1.0
}

impl<'de> Visitor<'de> for VelocityVisitor {
    type Value = Velocity;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a velocity represented as three f64 values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let first_byte: u8 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("expected first byte of velocity"))?;
        if first_byte == 0 {
            return Ok(Velocity::zero());
        }

        let second_byte: u8 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("expected second byte of velocity"))?;

        let remaining_bytes: u32 = seq
            .next_element()?
            .ok_or_else(|| de::Error::custom("expected remaining bytes of velocity"))?;

        let packed =
            (remaining_bytes as u64) << 16 | (second_byte as u64) << 8 | (first_byte as u64);

        let mut scale_factor = (first_byte & 0b11) as u64;
        if (first_byte & 4) == 4 {
            let extra: VarInt = seq
                .next_element()?
                .ok_or_else(|| de::Error::custom("expected extra bytes of velocity"))?;
            let extra = extra.0 as u64;
            scale_factor |= (extra & 0xFFFFFFFF) << 2;
        }

        let scale_factor = scale_factor as f64;
        let x = unpack(packed >> 3) * scale_factor;
        let y = unpack(packed >> 18) * scale_factor;
        let z = unpack(packed >> 33) * scale_factor;
        return Ok(Velocity::new(x, y, z));
    }
}

impl<'de> Deserialize<'de> for Velocity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(VelocityVisitor)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use serde::Deserialize;

    use crate::{messages::models::velocity::Velocity, ser::deserializer::Deserializer};

    fn deserialize_bytes(data: &[u8]) -> Velocity {
        let cursor = Cursor::new(data.to_vec());
        let mut deserializer = Deserializer::new(cursor);
        Velocity::deserialize(&mut deserializer).unwrap()
    }

    #[test]
    fn test_deserialize_zero_velocity() {
        assert_eq!(deserialize_bytes(&[0x00]), Velocity::zero());
    }

    #[test]
    fn test_deserialize_negative_velocity() {
        assert_eq!(
            deserialize_bytes(&[0xF1, 0xFF, 0x00, 0x00, 0xFF, 0xFF]),
            Velocity::new(1.0, 0.0, -1.0)
        );
    }

    #[test]
    fn test_7_byte_velocity() {
        assert_eq!(
            deserialize_bytes(&[0xF6, 0xFF, 0x40, 0x01, 0x05, 0x1F, 0x02]),
            Velocity::new(10.0, 0.2, -5.0)
        );
    }

    #[test]
    fn test_9_byte_velocity() {
        assert_eq!(
            deserialize_bytes(&[0xF5, 0xFF, 0x7F, 0xFF, 0x00, 0x07, 0x90, 0xF1, 0x01]),
            Velocity::new(123457.0, 15.071, 0.0)
        );
    }
}
