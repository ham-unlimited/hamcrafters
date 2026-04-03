use crate::{McPacket, codec::var_int::VarInt, messages::models::slot::Slot};
use mc_packet_macros::mc_packet;
use serde::{
    Deserialize,
    de::{self, SeqAccess, Visitor},
};
use std::fmt;

/// Clientbound set entity equipment packet during play phase.
#[derive(Debug)]
#[mc_packet(0x64)]
pub struct SetEntityEquipment {
    /// The entity ID.
    pub entity_id: VarInt,
    /// The equipment slots to set for the entity.
    pub equipment: Vec<Equipment>,
}

struct SetEntityEquipmentVisitor;

impl<'de> Visitor<'de> for SetEntityEquipmentVisitor {
    type Value = SetEntityEquipment;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of entity ID followed by equipment slots")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let entity_id = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

        let mut equipment = Vec::new();
        loop {
            let slot_byte: u8 = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(equipment.len() + 1, &self))?;
            let has_more = (slot_byte & 0x80) != 0;
            let slot = EquipmentSlot::try_from(slot_byte & 0x7F).map_err(de::Error::custom)?;
            let item = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(equipment.len() + 2, &self))?;
            equipment.push(Equipment { slot, item });
            if !has_more {
                break;
            }
        }

        Ok(SetEntityEquipment {
            entity_id,
            equipment,
        })
    }
}

impl<'de> Deserialize<'de> for SetEntityEquipment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_seq(SetEntityEquipmentVisitor)
    }
}

/// Represents a single piece of equipment for an entity, including the slot and the item in that slot.
#[derive(Debug)]
pub struct Equipment {
    /// The equipment slot being set.
    pub slot: EquipmentSlot,
    /// The item being equipped in that slot.
    pub item: Slot,
}

/// Represents the various equipment slots that can be set for an entity.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum EquipmentSlot {
    MainHand = 0,
    OffHand = 1,
    Boots = 2,
    Leggings = 3,
    Chestplate = 4,
    Helmet = 5,
    Body = 6,
    Saddle = 7,
}

impl TryFrom<u8> for EquipmentSlot {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EquipmentSlot::MainHand),
            1 => Ok(EquipmentSlot::OffHand),
            2 => Ok(EquipmentSlot::Boots),
            3 => Ok(EquipmentSlot::Leggings),
            4 => Ok(EquipmentSlot::Chestplate),
            5 => Ok(EquipmentSlot::Helmet),
            6 => Ok(EquipmentSlot::Body),
            7 => Ok(EquipmentSlot::Saddle),
            _ => Err(format!("unknown equipment slot: {value}")),
        }
    }
}
