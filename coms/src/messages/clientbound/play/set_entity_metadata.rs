use crate::{
    McPacket,
    codec::{
        identifier::Identifier, mc_uuid::McUuid, prefixed_array::PrefixedArray,
        prefixed_optional::PrefixedOptional, var_int::VarInt, var_long::VarLong,
    },
    messages::models::{position::Position, slot::Slot, text_component::TextComponent},
};
use mc_packet_macros::mc_packet;
use serde::{
    Deserialize,
    de::{self, SeqAccess, Visitor},
};
use std::fmt;

/// Clientbound set entity metadata packet during play phase.
#[derive(Debug, Deserialize)]
#[mc_packet(0x61)]
pub struct SetEntityMetadata {
    /// The entity ID.
    pub entity_id: VarInt,
    /// The metadata value to set for the entity.
    pub metadata: EntityMetadata,
}

/// A single entity metadata entry (index + typed value).
#[derive(Debug)]
pub struct EntityMetadata {
    /// Unique index key determining the meaning of the following value, see the table below. If this is 0xff then the it is the end of the Entity Metadata array and no more is read.
    pub index: u8,
    /// Only if Index is not 0xff; the type of the index, see the table below
    pub metadata_type: VarInt,
    /// The value of the metadata, the type of which is determined by the Metadata Type field.
    pub metadata_value: EntityMetadataValue,
}

impl<'de> Deserialize<'de> for EntityMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EntityMetadataVisitor;

        impl<'de> Visitor<'de> for EntityMetadataVisitor {
            type Value = EntityMetadata;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an entity metadata entry")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                fn next_required<'de, T, A>(
                    seq: &mut A,
                    field_name: &'static str,
                ) -> Result<T, A::Error>
                where
                    T: Deserialize<'de>,
                    A: SeqAccess<'de>,
                {
                    seq.next_element()?.ok_or_else(|| {
                        de::Error::custom(format!("missing {field_name} in entity metadata entry"))
                    })
                }

                let index: u8 = next_required(&mut seq, "metadata index")?;
                if index == u8::MAX {
                    return Err(de::Error::custom(
                        "entity metadata terminator (0xff) cannot be decoded as a standalone entry",
                    ));
                }

                let metadata_type: VarInt = next_required(&mut seq, "metadata type")?;
                let metadata_value = match metadata_type.0 {
                    0 => EntityMetadataValue::Byte(next_required(&mut seq, "byte metadata value")?),
                    1 => EntityMetadataValue::VarInt(next_required(
                        &mut seq,
                        "varint metadata value",
                    )?),
                    2 => EntityMetadataValue::VarLong(next_required(
                        &mut seq,
                        "varlong metadata value",
                    )?),
                    3 => {
                        EntityMetadataValue::Float(next_required(&mut seq, "float metadata value")?)
                    }
                    4 => EntityMetadataValue::String(next_required(
                        &mut seq,
                        "string metadata value",
                    )?),
                    5 => EntityMetadataValue::TextComponent(next_required(
                        &mut seq,
                        "text component metadata value",
                    )?),
                    6 => EntityMetadataValue::OptionalTextComponent(next_required(
                        &mut seq,
                        "optional text component metadata value",
                    )?),
                    7 => EntityMetadataValue::Slot(next_required(&mut seq, "slot metadata value")?),
                    8 => EntityMetadataValue::Boolean(next_required(
                        &mut seq,
                        "boolean metadata value",
                    )?),
                    9 => {
                        let x = next_required(&mut seq, "rotation x")?;
                        let y = next_required(&mut seq, "rotation y")?;
                        let z = next_required(&mut seq, "rotation z")?;
                        EntityMetadataValue::Rotations(x, y, z)
                    }
                    10 => EntityMetadataValue::Position(next_required(
                        &mut seq,
                        "position metadata value",
                    )?),
                    11 => EntityMetadataValue::OptionalPosition(next_required(
                        &mut seq,
                        "optional position metadata value",
                    )?),
                    12 => EntityMetadataValue::Direction(next_required(
                        &mut seq,
                        "direction metadata value",
                    )?),
                    13 => EntityMetadataValue::OptionalLivingEntityRef(next_required(
                        &mut seq,
                        "optional living entity reference metadata value",
                    )?),
                    14 => EntityMetadataValue::BlockState(next_required(
                        &mut seq,
                        "block state metadata value",
                    )?),
                    15 => {
                        let block_state: VarInt =
                            next_required(&mut seq, "optional block state metadata value")?;
                        EntityMetadataValue::OptionalBlockState(
                            (block_state.0 != 0).then_some(block_state),
                        )
                    }
                    16 => {
                        return Err(de::Error::custom(
                            "particle metadata values are not implemented yet",
                        ));
                    }
                    17 => {
                        return Err(de::Error::custom(
                            "particle array metadata values are not implemented yet",
                        ));
                    }
                    18 => EntityMetadataValue::VillagerData(next_required(
                        &mut seq,
                        "villager data metadata value",
                    )?),
                    19 => {
                        let raw_value: VarInt =
                            next_required(&mut seq, "optional varint metadata value")?;
                        EntityMetadataValue::OptionalVarInt(if raw_value.0 == 0 {
                            None
                        } else {
                            Some(VarInt(raw_value.0 - 1))
                        })
                    }
                    20 => {
                        EntityMetadataValue::Pose(next_required(&mut seq, "pose metadata value")?)
                    }
                    21 => EntityMetadataValue::CatVariant(next_required(
                        &mut seq,
                        "cat variant metadata value",
                    )?),
                    22 => EntityMetadataValue::CatSoundVariant(next_required(
                        &mut seq,
                        "cat sound variant metadata value",
                    )?),
                    23 => EntityMetadataValue::CowVariant(next_required(
                        &mut seq,
                        "cow variant metadata value",
                    )?),
                    24 => EntityMetadataValue::CowSoundVariant(next_required(
                        &mut seq,
                        "cow sound variant metadata value",
                    )?),
                    25 => EntityMetadataValue::WolfVariant(next_required(
                        &mut seq,
                        "wolf variant metadata value",
                    )?),
                    26 => EntityMetadataValue::WolfSoundVariant(next_required(
                        &mut seq,
                        "wolf sound variant metadata value",
                    )?),
                    27 => EntityMetadataValue::FrogVariant(next_required(
                        &mut seq,
                        "frog variant metadata value",
                    )?),
                    28 => EntityMetadataValue::PigVariant(next_required(
                        &mut seq,
                        "pig variant metadata value",
                    )?),
                    29 => EntityMetadataValue::PigSoundVariant(next_required(
                        &mut seq,
                        "pig sound variant metadata value",
                    )?),
                    30 => EntityMetadataValue::ChickenVariant(next_required(
                        &mut seq,
                        "chicken variant metadata value",
                    )?),
                    31 => EntityMetadataValue::ChickenSoundVariant(next_required(
                        &mut seq,
                        "chicken sound variant metadata value",
                    )?),
                    32 => EntityMetadataValue::ZombieNautilusVariant(next_required(
                        &mut seq,
                        "zombie nautilus variant metadata value",
                    )?),
                    33 => EntityMetadataValue::OptionalGlobalPosition(next_required(
                        &mut seq,
                        "optional global position metadata value",
                    )?),
                    34 => {
                        let holder_id: VarInt =
                            next_required(&mut seq, "painting variant holder metadata value")?;
                        let painting_variant = if holder_id.0 == 0 {
                            PaintingVariant::Inline {
                                width: next_required(&mut seq, "painting width")?,
                                height: next_required(&mut seq, "painting height")?,
                                asset_id: next_required(&mut seq, "painting asset id")?,
                                title: next_required(&mut seq, "painting title")?,
                                author: next_required(&mut seq, "painting author")?,
                            }
                        } else if holder_id.0 > 0 {
                            PaintingVariant::Id(VarInt(holder_id.0 - 1))
                        } else {
                            return Err(de::Error::custom(format!(
                                "invalid painting variant holder id {}",
                                holder_id.0
                            )));
                        };

                        EntityMetadataValue::PaintingVariant(painting_variant)
                    }
                    35 => EntityMetadataValue::SnifferState(next_required(
                        &mut seq,
                        "sniffer state metadata value",
                    )?),
                    36 => EntityMetadataValue::ArmadilloState(next_required(
                        &mut seq,
                        "armadillo state metadata value",
                    )?),
                    37 => EntityMetadataValue::CopperGolemState(next_required(
                        &mut seq,
                        "copper golem state metadata value",
                    )?),
                    38 => EntityMetadataValue::WeatheringCopperState(next_required(
                        &mut seq,
                        "weathering copper state metadata value",
                    )?),
                    39 => {
                        let x = next_required(&mut seq, "vector x")?;
                        let y = next_required(&mut seq, "vector y")?;
                        let z = next_required(&mut seq, "vector z")?;
                        EntityMetadataValue::Vector3(x, y, z)
                    }
                    40 => {
                        let x = next_required(&mut seq, "quaternion x")?;
                        let y = next_required(&mut seq, "quaternion y")?;
                        let z = next_required(&mut seq, "quaternion z")?;
                        let w = next_required(&mut seq, "quaternion w")?;
                        EntityMetadataValue::Quaternion(x, y, z, w)
                    }
                    41 => EntityMetadataValue::ResolvableProfile(next_required(
                        &mut seq,
                        "resolvable profile metadata value",
                    )?),
                    42 => EntityMetadataValue::HumanoidArm(next_required(
                        &mut seq,
                        "humanoid arm metadata value",
                    )?),
                    other => {
                        return Err(de::Error::custom(format!(
                            "unknown entity metadata type {other}"
                        )));
                    }
                };

                Ok(EntityMetadata {
                    index,
                    metadata_type,
                    metadata_value,
                })
            }
        }

        deserializer.deserialize_tuple(3, EntityMetadataVisitor)
    }
}

/// The direction an entity is facing.
#[derive(Debug, Deserialize)]
pub enum Direction {
    /// Facing downward.
    Down = 0,
    /// Facing upward.
    Up = 1,
    /// Facing north.
    North = 2,
    /// Facing south.
    South = 3,
    /// Facing west.
    West = 4,
    /// Facing east.
    East = 5,
}

/// The pose of an entity.
#[derive(Debug, Deserialize)]
pub enum Pose {
    /// Standing upright.
    Standing = 0,
    /// Flying with elytra.
    FallFlying = 1,
    /// Sleeping in a bed.
    Sleeping = 2,
    /// Swimming.
    Swimming = 3,
    /// Performing a riptide spin attack.
    SpinAttack = 4,
    /// Sneaking.
    Sneaking = 5,
    /// Long jumping (goat).
    LongJumping = 6,
    /// Dying.
    Dying = 7,
    /// Croaking (frog).
    Croaking = 8,
    /// Using tongue (frog).
    UsingTongue = 9,
    /// Sitting.
    Sitting = 10,
    /// Roaring (warden).
    Roaring = 11,
    /// Sniffing (sniffer).
    Sniffing = 12,
    /// Emerging from the ground (sniffer).
    Emerging = 13,
    /// Digging (sniffer).
    Digging = 14,
    /// Sliding.
    Sliding = 15,
    /// Shooting.
    Shooting = 16,
    /// Inhaling.
    Inhaling = 17,
}

/// The state of a Sniffer entity.
#[derive(Debug, Deserialize)]
pub enum SnifferState {
    /// Idling.
    Idling = 0,
    /// Feeling happy.
    FeelingHappy = 1,
    /// Scenting.
    Scenting = 2,
    /// Sniffing.
    Sniffing = 3,
    /// Searching.
    Searching = 4,
    /// Digging.
    Digging = 5,
    /// Rising from the ground.
    Rising = 6,
}

/// The state of an Armadillo entity.
#[derive(Debug, Deserialize)]
pub enum ArmadilloState {
    /// Idle, not rolled up.
    Idle = 0,
    /// Rolling into a ball.
    Rolling = 1,
    /// Scared and rolled up.
    Scared = 2,
    /// Unrolling from a ball.
    Unrolling = 3,
}

/// The state of a Copper Golem entity.
#[derive(Debug, Deserialize)]
pub enum CopperGolemState {
    /// Idle.
    Idle = 0,
    /// Pressing a button with an item.
    GettingItem = 1,
    /// Pressing a button without an item.
    GettingNoItem = 2,
    /// Dropping an item.
    DroppingItem = 3,
    /// Dropping nothing.
    DroppingNoItem = 4,
}

/// The weathering state of a copper block/entity.
#[derive(Debug, Deserialize)]
pub enum WeatheringCopperState {
    /// Unweathered copper.
    Unaffected = 0,
    /// Exposed copper.
    Exposed = 1,
    /// Weathered copper.
    Weathered = 2,
    /// Fully oxidized copper.
    Oxidized = 3,
}

/// The main hand of a humanoid entity.
#[derive(Debug, Deserialize)]
pub enum HumanoidArm {
    /// Left arm.
    Left = 0,
    /// Right arm.
    Right = 1,
}

/// A painting variant: either an ID in the painting_variant registry or an inline definition.
#[derive(Debug)]
pub enum PaintingVariant {
    /// An ID in the minecraft:painting_variant registry.
    Id(VarInt),
    /// An inline painting variant definition.
    Inline {
        /// The width of the painting in blocks.
        width: VarInt,
        /// The height of the painting in blocks.
        height: VarInt,
        /// The texture asset ID for the painting.
        asset_id: Identifier,
        /// The displayed title, if present.
        title: PrefixedOptional<TextComponent>,
        /// The displayed author, if present.
        author: PrefixedOptional<TextComponent>,
    },
}

/// Villager type, profession, and level.
#[derive(Debug, Deserialize)]
pub struct VillagerData {
    /// The villager type (VarInt ID in minecraft:villager_type registry).
    pub villager_type: VarInt,
    /// The villager profession (VarInt ID in minecraft:villager_profession registry).
    pub profession: VarInt,
    /// The villager level.
    pub level: VarInt,
}

/// A single particle definition: particle type ID and its associated data.
#[derive(Debug)]
pub struct Particle {
    /// The particle type ID (in minecraft:particle_type registry).
    pub particle_type: VarInt,
    /// The particle-specific data (format varies by particle type).
    pub data: Vec<u8>,
}

/// Texture and model overrides for a resolvable profile.
#[derive(Debug, Deserialize)]
pub struct ProfileOverrides {
    /// Skin texture override (from the player's textures directory).
    pub body: PrefixedOptional<Identifier>,
    /// Cape texture override.
    pub cape: PrefixedOptional<Identifier>,
    /// Elytra texture override.
    pub elytra: PrefixedOptional<Identifier>,
    /// Model override (0 = wide, 1 = slim).
    pub model: PrefixedOptional<VarInt>,
}

/// A resolvable player profile (either partial or complete).
#[derive(Debug)]
pub enum ResolvableProfile {
    /// An unresolved partial profile with no game profile data.
    Partial {
        /// Texture and model overrides.
        overrides: ProfileOverrides,
    },
    /// A fully resolved player profile.
    Complete {
        /// The player's username.
        name: String,
        /// The player's UUID.
        uuid: McUuid,
        /// The profile properties (e.g. skin texture).
        properties: PrefixedArray<ProfileProperty>,
        /// Texture and model overrides.
        overrides: ProfileOverrides,
    },
}

impl<'de> Deserialize<'de> for ResolvableProfile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResolvableProfileVisitor;

        impl<'de> Visitor<'de> for ResolvableProfileVisitor {
            type Value = ResolvableProfile;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a resolvable profile")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                fn next_required<'de, T, A>(
                    seq: &mut A,
                    field_name: &'static str,
                ) -> Result<T, A::Error>
                where
                    T: Deserialize<'de>,
                    A: SeqAccess<'de>,
                {
                    seq.next_element()?.ok_or_else(|| {
                        de::Error::custom(format!("missing {field_name} in resolvable profile"))
                    })
                }

                let kind: VarInt = next_required(&mut seq, "profile kind")?;
                match kind.0 {
                    0 => Ok(ResolvableProfile::Partial {
                        overrides: next_required(&mut seq, "profile overrides")?,
                    }),
                    1 => {
                        let name: String = next_required(&mut seq, "profile name")?;
                        let uuid: McUuid = next_required(&mut seq, "profile uuid")?;
                        let properties: PrefixedArray<ProfileProperty> =
                            next_required(&mut seq, "profile properties")?;
                        let overrides: ProfileOverrides =
                            next_required(&mut seq, "profile overrides")?;
                        Ok(ResolvableProfile::Complete {
                            name,
                            uuid,
                            properties,
                            overrides,
                        })
                    }
                    other => Err(de::Error::custom(format!(
                        "unknown resolvable profile kind {other}"
                    ))),
                }
            }
        }

        deserializer.deserialize_tuple(8, ResolvableProfileVisitor)
    }
}

/// A single property in a player profile (e.g. textures).
#[derive(Debug, Deserialize)]
pub struct ProfileProperty {
    /// The property name (e.g. "textures").
    pub name: String,
    /// The property value (base64-encoded JSON).
    pub value: String,
    /// The Yggdrasil signature for this property, if present.
    pub signature: PrefixedOptional<String>,
}

/// The value of an entity metadata field.
/// The type is determined by the metadata type VarInt field.
/// See: <https://minecraft.wiki/w/Java_Edition_protocol/Entity_metadata#Entity_Metadata_Format>
#[derive(Debug)]
pub enum EntityMetadataValue {
    /// Type 0: Byte
    Byte(u8),
    /// Type 1: VarInt
    VarInt(VarInt),
    /// Type 2: VarLong
    VarLong(VarLong),
    /// Type 3: Float
    Float(f32),
    /// Type 4: String (up to 32767 characters)
    String(String),
    /// Type 5: Text Component
    TextComponent(TextComponent),
    /// Type 6: Optional Text Component
    OptionalTextComponent(PrefixedOptional<TextComponent>),
    /// Type 7: Slot
    Slot(Slot),
    /// Type 8: Boolean
    Boolean(bool),
    /// Type 9: Rotations (x, y, z in degrees)
    Rotations(f32, f32, f32),
    /// Type 10: Position
    Position(Position),
    /// Type 11: Optional Position
    OptionalPosition(PrefixedOptional<Position>),
    /// Type 12: Direction (VarInt enum: Down=0, Up=1, North=2, South=3, West=4, East=5)
    Direction(Direction),
    /// Type 13: Optional Living Entity Reference (Optional UUID)
    OptionalLivingEntityRef(PrefixedOptional<McUuid>),
    /// Type 14: Block State (ID in the block state registry)
    BlockState(VarInt),
    /// Type 15: Optional Block State (0 for absent/air; otherwise an ID in the block state registry)
    OptionalBlockState(Option<VarInt>),
    /// Type 16: Particle (particle type ID + particle-specific data id in the minecraft:particle_type registry).
    Particle(Particle),
    /// Type 17: Particles (length-prefixed list of particle definitions)
    Particles(PrefixedArray<Particle>),
    /// Type 18: Villager Data (type, profession, level)
    VillagerData(VillagerData),
    /// Type 19: Optional VarInt (0 for absent; 1 + actual value otherwise; used for entity IDs)
    OptionalVarInt(Option<VarInt>),
    /// Type 20: Pose (VarInt enum)
    Pose(Pose),
    /// Type 21: Cat Variant (ID in minecraft:cat_variant registry)
    CatVariant(VarInt),
    /// Type 22: Cat Sound Variant (ID in minecraft:cat_sound_variant registry)
    CatSoundVariant(VarInt),
    /// Type 23: Cow Variant (ID in minecraft:cow_variant registry)
    CowVariant(VarInt),
    /// Type 24: Cow Sound Variant (ID in minecraft:cow_sound_variant registry)
    CowSoundVariant(VarInt),
    /// Type 25: Wolf Variant (ID in minecraft:wolf_variant registry)
    WolfVariant(VarInt),
    /// Type 26: Wolf Sound Variant (ID in minecraft:wolf_sound_variant registry)
    WolfSoundVariant(VarInt),
    /// Type 27: Frog Variant (ID in minecraft:frog_variant registry)
    FrogVariant(VarInt),
    /// Type 28: Pig Variant (ID in minecraft:pig_variant registry)
    PigVariant(VarInt),
    /// Type 29: Pig Sound Variant (ID in minecraft:pig_sound_variant registry)
    PigSoundVariant(VarInt),
    /// Type 30: Chicken Variant (ID in minecraft:chicken_variant registry)
    ChickenVariant(VarInt),
    /// Type 31: Chicken Sound Variant (ID in minecraft:chicken_sound_variant registry)
    ChickenSoundVariant(VarInt),
    /// Type 32: Zombie Nautilus Variant (ID in minecraft:zombie_nautilus_variant registry)
    ZombieNautilusVariant(VarInt),
    /// Type 33: Optional Global Position (dimension identifier + block position; absent if boolean is false)
    OptionalGlobalPosition(PrefixedOptional<(Identifier, Position)>),
    /// Type 34: Painting Variant (ID in minecraft:painting_variant registry, or an inline definition)
    PaintingVariant(PaintingVariant),
    /// Type 35: Sniffer State (VarInt enum)
    SnifferState(SnifferState),
    /// Type 36: Armadillo State (VarInt enum)
    ArmadilloState(ArmadilloState),
    /// Type 37: Copper Golem State (VarInt enum)
    CopperGolemState(CopperGolemState),
    /// Type 38: Weathering Copper State (VarInt enum)
    WeatheringCopperState(WeatheringCopperState),
    /// Type 39: Vector3 (x, y, z)
    Vector3(f32, f32, f32),
    /// Type 40: Quaternion (x, y, z, w)
    Quaternion(f32, f32, f32, f32),
    /// Type 41: Resolvable Profile
    ResolvableProfile(ResolvableProfile),
    /// Type 42: Humanoid Arm (VarInt enum: Left=0, Right=1)
    HumanoidArm(HumanoidArm),
}
