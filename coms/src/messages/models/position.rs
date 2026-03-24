use serde::{Deserialize, Serialize};

// TODO: MOVE SOMEWHERE MORE RELEVANT.
// TODO: Test this? E.g. 0100011000000111011000110010110000010101101101001000001100111111 should be x = 18357644, y = 831 and z = -20882616.
/// Represents a position in the Minecraft world.
#[derive(Debug)]
pub struct Position {
    /// The X coordinate of the position (26-bit).
    pub x: i32,
    /// The Y coordinate of the position (12-bit).
    pub y: i16,
    /// The Z coordinate of the position (26-bit).
    pub z: i32,
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;

        let x = (value >> 38) as i32;
        let y = (value << 52 >> 52) as i16;
        let z = (value << 26 >> 38) as i32;
        Ok(Position { x, y, z })
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = ((self.x as i64 & 0x3FFFFFF) << 38)
            | ((self.z as i64 & 0x3FFFFFF) << 12)
            | (self.y as i64 & 0xFFF);
        value.serialize(serializer)
    }
}
