use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
};
use mc_packet_macros::mc_packet;
use serde::Deserialize;

/// Clientbound Update Attributes packet during play phase.
#[derive(Debug, Deserialize)]
#[mc_packet(0x81)]
pub struct UpdateAttributes {
    /// The ID of the entity whose attributes are being updated.
    pub entity_id: VarInt,
    /// The attributes being updated.
    pub properties: PrefixedArray<AttributeProperty>,
}

/// An attribute of an entity, such as max health or movement speed.
#[derive(Debug, Deserialize)]
pub struct AttributeProperty {
    /// The ID of the attribute being updated, as defined in the minecraft:attribute registry.
    pub id: VarInt,
    /// The value of the attribute.
    // TODO: Figure out of this is before or after applying the modifiers?
    pub value: f64,
    /// The modifiers to apply to the attribute.
    pub modifiers: PrefixedArray<AttributeModifier>,
}

/// A modifier to apply to an attribute, such as adding 10 to max health or multiplying movement speed by 1.5.
#[derive(Debug, Deserialize)]
pub struct AttributeModifier {
    /// The ID of the modifier.
    pub id: Identifier,
    /// The amount to apply the modifier with.
    pub amount: f64,
    /// The operation to apply the modifier with.
    pub operation: AttributeModifierOperation,
}

/// These are applied in order of the operation so all AddSubtract are applied first, then all AddSubtractPercent, and finally all Multiply.
#[derive(Debug)]
pub enum AttributeModifierOperation {
    /// Add/subtract amount, highest prio.
    AddSubtract,
    /// Add/subtract amount percent of the current value, second highest prio.
    AddSubtractPercent,
    /// Multiply by amount percent, lowest prio.
    Multiply,
}

impl<'de> Deserialize<'de> for AttributeModifierOperation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let operation: u8 = u8::deserialize(deserializer)?;

        match operation {
            0 => Ok(Self::AddSubtract),
            1 => Ok(Self::AddSubtractPercent),
            2 => Ok(Self::Multiply),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid attribute modifier operation: {operation}"
            ))),
        }
    }
}
