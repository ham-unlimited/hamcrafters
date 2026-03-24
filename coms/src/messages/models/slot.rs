use nbt::nbt_types::NbtCompound;
use serde::{Deserialize, Serialize};

use crate::{codec::var_int::VarInt, messages::models::text_component::TextComponent};

/// A minecraft slot. Defines how an item is represented in an inventory of any kind.
#[derive(Debug)]
pub enum Slot {
    /// Empty slot, no item is present.
    Empty,
    /// Slot containing an item.
    SlotContent(SlotContent),
}

impl<'de> Deserialize<'de> for Slot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{SeqAccess, Visitor};

        struct SlotVisitor;

        impl<'de> Visitor<'de> for SlotVisitor {
            type Value = Slot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a slot")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let item_count: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("missing item count"))?;

                if item_count.0 == 0 {
                    return Ok(Slot::Empty);
                }

                if item_count.0 < 0 {
                    return Err(serde::de::Error::custom(format!(
                        "Invalid item count in slot: {}",
                        item_count.0
                    )));
                }

                let item_id: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("missing item id"))?;

                let num_to_add: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("missing component-to-add count"))?;

                let num_to_remove: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("missing component-to-remove count"))?;

                let mut components_to_add = Vec::with_capacity(num_to_add.0 as usize);
                for _ in 0..num_to_add.0 {
                    let component = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::custom("missing component to add"))?;
                    components_to_add.push(component);
                }

                let mut components_to_remove = Vec::with_capacity(num_to_remove.0 as usize);
                for _ in 0..num_to_remove.0 {
                    let component = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::custom("missing component to remove"))?;
                    components_to_remove.push(component);
                }

                Ok(Slot::SlotContent(SlotContent {
                    item_count,
                    item_id,
                    components_to_add,
                    components_to_remove,
                }))
            }
        }

        deserializer.deserialize_seq(SlotVisitor)
    }
}

impl Serialize for Slot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Slot::Empty => VarInt::from(0).serialize(serializer),
            Slot::SlotContent(slot_content) => slot_content.serialize(serializer),
        }
    }
}

/// Content of an inventory slot.
#[derive(Debug)]
pub struct SlotContent {
    /// The number of items in this slot content.
    pub item_count: VarInt,
    /// The ID of the item in the slot. This is an ID in the "minecraft:item" registry.
    /// Note Item IDs are different from Block IDs.
    pub item_id: VarInt,
    /// Components to add to the slot. These are used to add additional data to a slot, such as enchantments, custom names, etc.
    pub components_to_add: Vec<ComponentData>,
    /// Components to remove from the slot.
    pub components_to_remove: Vec<ComponentType>,
}

impl Serialize for SlotContent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;
        let n = 4 + self.components_to_add.len() + self.components_to_remove.len();
        let mut tup = serializer.serialize_tuple(n)?;
        tup.serialize_element(&self.item_count)?;
        tup.serialize_element(&self.item_id)?;
        tup.serialize_element(&VarInt(self.components_to_add.len() as i32))?;
        tup.serialize_element(&VarInt(self.components_to_remove.len() as i32))?;
        for c in &self.components_to_add {
            tup.serialize_element(c)?;
        }
        for c in &self.components_to_remove {
            tup.serialize_element(c)?;
        }
        tup.end()
    }
}

/// The type ID of a component, used in the components-to-remove list.
///
/// This is a VarInt index into the "minecraft:data_component_type" registry.
pub type ComponentType = VarInt;

/// All structured data component types for a slot item.
///
/// Each variant represents one component type and carries its payload.
/// The discriminant VarInt is written/read as part of serialization.
///
/// See: <https://minecraft.wiki/w/Java_Edition_protocol/Slot_data#Structured_components>
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum ComponentData {
    /// 0: minecraft:custom_data
    CustomData(NbtCompound),
    /// 1: minecraft:max_stack_size
    MaxStackSize(VarInt),
    /// 2: minecraft:max_damage
    MaxDamage(VarInt),
    /// 3: minecraft:damage
    Damage(VarInt),
    /// 4: minecraft:unbreakable — no fields
    Unbreakable,
    /// 5: minecraft:custom_name — Text Component (NBT tag)
    CustomName(TextComponent),
    /// 6: minecraft:item_name — Text Component (NBT tag)
    ItemName(TextComponent),
    /// 7: minecraft:item_model — Identifier
    ItemModel(String),
    /// 8: minecraft:lore — Prefixed Array of Text Component
    Lore, // TODO
    /// 9: minecraft:rarity — VarInt Enum (0: common, 1: uncommon, 2: rare, 3: epic)
    Rarity(VarInt),
    /// 10: minecraft:enchantments
    Enchantments, // TODO
    /// 11: minecraft:can_place_on
    CanPlaceOn, // TODO
    /// 12: minecraft:can_break
    CanBreak, // TODO
    /// 13: minecraft:attribute_modifiers
    AttributeModifiers, // TODO
    /// 14: minecraft:custom_model_data
    CustomModelData, // TODO
    /// 15: minecraft:tooltip_display
    TooltipDisplay, // TODO
    /// 16: minecraft:repair_cost
    RepairCost(VarInt),
    /// 17: minecraft:creative_slot_lock — no fields
    CreativeSlotLock,
    /// 18: minecraft:enchantment_glint_override
    EnchantmentGlintOverride(bool),
    /// 19: minecraft:intangible_projectile
    IntangibleProjectile, // TODO
    /// 20: minecraft:food
    Food, // TODO
    /// 21: minecraft:consumable
    Consumable, // TODO
    /// 22: minecraft:use_remainder — Slot
    UseRemainder(Box<Slot>),
    /// 23: minecraft:use_cooldown
    UseCooldown, // TODO
    /// 24: minecraft:damage_resistant
    DamageResistant, // TODO
    /// 25: minecraft:tool
    Tool, // TODO
    /// 26: minecraft:weapon
    Weapon, // TODO
    /// 27: minecraft:enchantable
    Enchantable(VarInt),
    /// 28: minecraft:equippable
    Equippable, // TODO
    /// 29: minecraft:repairable
    Repairable, // TODO
    /// 30: minecraft:glider — no fields
    Glider,
    /// 31: minecraft:tooltip_style — Identifier
    TooltipStyle(String),
    /// 32: minecraft:death_protection
    DeathProtection, // TODO
    /// 33: minecraft:blocks_attacks
    BlocksAttacks, // TODO
    /// 34: minecraft:stored_enchantments
    StoredEnchantments, // TODO
    /// 35: minecraft:dyed_color
    DyedColor, // TODO
    /// 36: minecraft:map_color — Int (big-endian i32)
    MapColor(i32),
    /// 37: minecraft:map_id
    MapId(VarInt),
    /// 38: minecraft:map_decorations — NBT Compound
    MapDecorations(NbtCompound),
    /// 39: minecraft:map_post_processing — VarInt Enum (0: lock, 1: scale)
    MapPostProcessing(VarInt),
    /// 40: minecraft:charged_projectiles — Prefixed Array of Slot
    ChargedProjectiles(Vec<Slot>),
    /// 41: minecraft:bundle_contents — Prefixed Array of Slot
    BundleContents(Vec<Slot>),
    /// 42: minecraft:potion_contents
    PotionContents, // TODO
    /// 43: minecraft:potion_duration_scale — Float
    PotionDurationScale(f32),
    /// 44: minecraft:suspicious_stew_effects
    SuspiciousStewEffects, // TODO
    /// 45: minecraft:writable_book_content
    WritableBookContent, // TODO
    /// 46: minecraft:written_book_content
    WrittenBookContent, // TODO
    /// 47: minecraft:trim
    Trim, // TODO
    /// 48: minecraft:debug_stick_state — NBT Compound
    DebugStickState(NbtCompound),
    /// 49: minecraft:entity_data — NBT Compound
    EntityData(NbtCompound),
    /// 50: minecraft:bucket_entity_data — NBT Compound
    BucketEntityData(NbtCompound),
    /// 51: minecraft:block_entity_data — NBT Compound
    BlockEntityData(NbtCompound),
    /// 52: minecraft:instrument
    Instrument, // TODO
    /// 53: minecraft:provides_trim_material
    ProvidesTrimMaterial, // TODO
    /// 54: minecraft:ominous_bottle_amplifier
    OminousBottleAmplifier(VarInt),
    /// 55: minecraft:jukebox_playable
    JukeboxPlayable, // TODO
    /// 56: minecraft:provides_banner_patterns — Identifier
    ProvidesBannerPatterns(String),
    /// 57: minecraft:recipes — NBT Compound
    Recipes(NbtCompound),
    /// 58: minecraft:lodestone_tracker
    LodestoneTracker, // TODO
    /// 59: minecraft:firework_explosion
    FireworkExplosion, // TODO
    /// 60: minecraft:fireworks
    Fireworks, // TODO
    /// 61: minecraft:profile — Game Profile
    Profile, // TODO
    /// 62: minecraft:note_block_sound — Identifier
    NoteBlockSound(String),
    /// 63: minecraft:banner_patterns
    BannerPatterns, // TODO
    /// 64: minecraft:base_color — VarInt Enum (DyeColor)
    BaseColor(VarInt),
    /// 65: minecraft:pot_decorations — Prefixed Array of VarInt
    PotDecorations(Vec<VarInt>),
    /// 66: minecraft:container — Prefixed Array of Slot
    Container(Vec<Slot>),
    /// 67: minecraft:block_state — Prefixed Array of (String, String)
    BlockState, // TODO
    /// 68: minecraft:bees
    Bees, // TODO
    /// 69: minecraft:lock — String
    Lock(String),
    /// 70: minecraft:container_loot
    ContainerLoot, // TODO
    /// 71: minecraft:break_sound — ID or Sound Event
    BreakSound, // TODO
    /// 72: minecraft:villager/variant — VarInt
    VillagerVariant(VarInt),
    /// 73: minecraft:wolf/variant
    WolfVariant, // TODO
    /// 74: minecraft:wolf/sound_variant
    WolfSoundVariant, // TODO
    /// 75: minecraft:wolf/collar — VarInt Enum (DyeColor)
    WolfCollar(VarInt),
    /// 76: minecraft:fox/variant — VarInt Enum
    FoxVariant(VarInt),
    /// 77: minecraft:salmon/size — VarInt Enum
    SalmonSize(VarInt),
    /// 78: minecraft:parrot/variant — VarInt Enum
    ParrotVariant(VarInt),
    /// 79: minecraft:tropical_fish/pattern — VarInt Enum
    TropicalFishPattern(VarInt),
    /// 80: minecraft:tropical_fish/base_color — VarInt Enum (DyeColor)
    TropicalFishBaseColor(VarInt),
    /// 81: minecraft:tropical_fish/pattern_color — VarInt Enum (DyeColor)
    TropicalFishPatternColor(VarInt),
    /// 82: minecraft:mooshroom/variant — VarInt Enum
    MooshroomVariant(VarInt),
    /// 83: minecraft:rabbit/variant — VarInt Enum
    RabbitVariant(VarInt),
    /// 84: minecraft:pig/variant
    PigVariant, // TODO
    /// 85: minecraft:cow/variant
    CowVariant, // TODO
    /// 86: minecraft:chicken/variant
    ChickenVariant, // TODO
    /// 87: minecraft:frog/variant
    FrogVariant, // TODO
    /// 88: minecraft:horse/variant — VarInt Enum
    HorseVariant(VarInt),
    /// 89: minecraft:painting/variant
    PaintingVariant, // TODO
    /// 90: minecraft:llama/variant — VarInt Enum
    LlamaVariant(VarInt),
    /// 91: minecraft:axolotl/variant — VarInt Enum
    AxolotlVariant(VarInt),
    /// 92: minecraft:cat/variant
    CatVariant, // TODO
    /// 93: minecraft:cat/collar — VarInt Enum (DyeColor)
    CatCollar(VarInt),
    /// 94: minecraft:sheep/color — VarInt Enum (DyeColor)
    SheepColor(VarInt),
    /// 95: minecraft:shulker/color — VarInt Enum (DyeColor)
    ShulkerColor(VarInt),
}

impl<'de> Deserialize<'de> for ComponentData {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{SeqAccess, Visitor};

        struct ComponentDataVisitor;

        impl<'de> Visitor<'de> for ComponentDataVisitor {
            type Value = ComponentData;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("component data")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let discriminant: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("missing component discriminant"))?;

                macro_rules! next {
                    () => {
                        seq.next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing component data"))?
                    };
                }
                macro_rules! todo_de {
                    ($name:literal) => {
                        Err(serde::de::Error::custom(concat!(
                            "ComponentData::",
                            $name,
                            " is not yet implemented"
                        )))
                    };
                }
                macro_rules! nbt_de {
                    ($name:literal) => {
                        Err(serde::de::Error::custom(concat!(
                            "ComponentData::",
                            $name,
                            " requires NBT deserialization, not yet implemented"
                        )))
                    };
                }
                macro_rules! prefixed_slots {
                    ($variant:expr) => {{
                        let count: VarInt = next!();
                        let mut slots = Vec::with_capacity(count.0 as usize);
                        for _ in 0..count.0 {
                            slots.push(next!());
                        }
                        Ok($variant(slots))
                    }};
                }

                match discriminant.0 {
                    0 => nbt_de!("CustomData"),
                    1 => Ok(ComponentData::MaxStackSize(next!())),
                    2 => Ok(ComponentData::MaxDamage(next!())),
                    3 => Ok(ComponentData::Damage(next!())),
                    4 => Ok(ComponentData::Unbreakable),
                    5 => todo_de!("CustomName"),
                    6 => todo_de!("ItemName"),
                    7 => Ok(ComponentData::ItemModel(next!())),
                    8 => todo_de!("Lore"),
                    9 => Ok(ComponentData::Rarity(next!())),
                    10 => todo_de!("Enchantments"),
                    11 => todo_de!("CanPlaceOn"),
                    12 => todo_de!("CanBreak"),
                    13 => todo_de!("AttributeModifiers"),
                    14 => todo_de!("CustomModelData"),
                    15 => todo_de!("TooltipDisplay"),
                    16 => Ok(ComponentData::RepairCost(next!())),
                    17 => Ok(ComponentData::CreativeSlotLock),
                    18 => Ok(ComponentData::EnchantmentGlintOverride(next!())),
                    19 => todo_de!("IntangibleProjectile"),
                    20 => todo_de!("Food"),
                    21 => todo_de!("Consumable"),
                    22 => Ok(ComponentData::UseRemainder(Box::new(next!()))),
                    23 => todo_de!("UseCooldown"),
                    24 => todo_de!("DamageResistant"),
                    25 => todo_de!("Tool"),
                    26 => todo_de!("Weapon"),
                    27 => Ok(ComponentData::Enchantable(next!())),
                    28 => todo_de!("Equippable"),
                    29 => todo_de!("Repairable"),
                    30 => Ok(ComponentData::Glider),
                    31 => Ok(ComponentData::TooltipStyle(next!())),
                    32 => todo_de!("DeathProtection"),
                    33 => todo_de!("BlocksAttacks"),
                    34 => todo_de!("StoredEnchantments"),
                    35 => todo_de!("DyedColor"),
                    36 => Ok(ComponentData::MapColor(next!())),
                    37 => Ok(ComponentData::MapId(next!())),
                    38 => nbt_de!("MapDecorations"),
                    39 => Ok(ComponentData::MapPostProcessing(next!())),
                    40 => prefixed_slots!(ComponentData::ChargedProjectiles),
                    41 => prefixed_slots!(ComponentData::BundleContents),
                    42 => todo_de!("PotionContents"),
                    43 => Ok(ComponentData::PotionDurationScale(next!())),
                    44 => todo_de!("SuspiciousStewEffects"),
                    45 => todo_de!("WritableBookContent"),
                    46 => todo_de!("WrittenBookContent"),
                    47 => todo_de!("Trim"),
                    48 => nbt_de!("DebugStickState"),
                    49 => nbt_de!("EntityData"),
                    50 => nbt_de!("BucketEntityData"),
                    51 => nbt_de!("BlockEntityData"),
                    52 => todo_de!("Instrument"),
                    53 => todo_de!("ProvidesTrimMaterial"),
                    54 => Ok(ComponentData::OminousBottleAmplifier(next!())),
                    55 => todo_de!("JukeboxPlayable"),
                    56 => Ok(ComponentData::ProvidesBannerPatterns(next!())),
                    57 => nbt_de!("Recipes"),
                    58 => todo_de!("LodestoneTracker"),
                    59 => todo_de!("FireworkExplosion"),
                    60 => todo_de!("Fireworks"),
                    61 => todo_de!("Profile"),
                    62 => Ok(ComponentData::NoteBlockSound(next!())),
                    63 => todo_de!("BannerPatterns"),
                    64 => Ok(ComponentData::BaseColor(next!())),
                    65 => {
                        let count: VarInt = next!();
                        let mut items = Vec::with_capacity(count.0 as usize);
                        for _ in 0..count.0 {
                            items.push(next!());
                        }
                        Ok(ComponentData::PotDecorations(items))
                    }
                    66 => prefixed_slots!(ComponentData::Container),
                    67 => todo_de!("BlockState"),
                    68 => todo_de!("Bees"),
                    69 => Ok(ComponentData::Lock(next!())),
                    70 => todo_de!("ContainerLoot"),
                    71 => todo_de!("BreakSound"),
                    72 => Ok(ComponentData::VillagerVariant(next!())),
                    73 => todo_de!("WolfVariant"),
                    74 => todo_de!("WolfSoundVariant"),
                    75 => Ok(ComponentData::WolfCollar(next!())),
                    76 => Ok(ComponentData::FoxVariant(next!())),
                    77 => Ok(ComponentData::SalmonSize(next!())),
                    78 => Ok(ComponentData::ParrotVariant(next!())),
                    79 => Ok(ComponentData::TropicalFishPattern(next!())),
                    80 => Ok(ComponentData::TropicalFishBaseColor(next!())),
                    81 => Ok(ComponentData::TropicalFishPatternColor(next!())),
                    82 => Ok(ComponentData::MooshroomVariant(next!())),
                    83 => Ok(ComponentData::RabbitVariant(next!())),
                    84 => todo_de!("PigVariant"),
                    85 => todo_de!("CowVariant"),
                    86 => todo_de!("ChickenVariant"),
                    87 => todo_de!("FrogVariant"),
                    88 => Ok(ComponentData::HorseVariant(next!())),
                    89 => todo_de!("PaintingVariant"),
                    90 => Ok(ComponentData::LlamaVariant(next!())),
                    91 => Ok(ComponentData::AxolotlVariant(next!())),
                    92 => todo_de!("CatVariant"),
                    93 => Ok(ComponentData::CatCollar(next!())),
                    94 => Ok(ComponentData::SheepColor(next!())),
                    95 => Ok(ComponentData::ShulkerColor(next!())),
                    n => Err(serde::de::Error::custom(format!(
                        "Unknown component type discriminant: {n}"
                    ))),
                }
            }
        }

        deserializer.deserialize_seq(ComponentDataVisitor)
    }
}

impl Serialize for ComponentData {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeTuple;

        // Serialize discriminant only (unit variants — no payload).
        macro_rules! unit {
            ($disc:literal) => {{ VarInt($disc).serialize(serializer) }};
        }
        // Serialize discriminant + one payload field.
        macro_rules! one {
            ($disc:literal, $val:expr) => {{
                let mut t = serializer.serialize_tuple(2)?;
                t.serialize_element(&VarInt($disc))?;
                t.serialize_element($val)?;
                t.end()
            }};
        }
        macro_rules! todo_ser {
            ($name:literal) => {{
                Err(serde::ser::Error::custom(concat!(
                    "ComponentData::",
                    $name,
                    " is not yet implemented"
                )))
            }};
        }
        macro_rules! nbt_ser {
            ($name:literal) => {{
                Err(serde::ser::Error::custom(concat!(
                    "ComponentData::",
                    $name,
                    " requires NBT serialization, not yet implemented"
                )))
            }};
        }

        match self {
            ComponentData::CustomData(_) => nbt_ser!("CustomData"),
            ComponentData::MaxStackSize(v) => one!(1, v),
            ComponentData::MaxDamage(v) => one!(2, v),
            ComponentData::Damage(v) => one!(3, v),
            ComponentData::Unbreakable => unit!(4),
            ComponentData::CustomName(_) => todo_ser!("CustomName"),
            ComponentData::ItemName(_) => todo_ser!("ItemName"),
            ComponentData::ItemModel(v) => one!(7, v),
            ComponentData::Lore => todo_ser!("Lore"),
            ComponentData::Rarity(v) => one!(9, v),
            ComponentData::Enchantments => todo_ser!("Enchantments"),
            ComponentData::CanPlaceOn => todo_ser!("CanPlaceOn"),
            ComponentData::CanBreak => todo_ser!("CanBreak"),
            ComponentData::AttributeModifiers => todo_ser!("AttributeModifiers"),
            ComponentData::CustomModelData => todo_ser!("CustomModelData"),
            ComponentData::TooltipDisplay => todo_ser!("TooltipDisplay"),
            ComponentData::RepairCost(v) => one!(16, v),
            ComponentData::CreativeSlotLock => unit!(17),
            ComponentData::EnchantmentGlintOverride(v) => one!(18, v),
            ComponentData::IntangibleProjectile => todo_ser!("IntangibleProjectile"),
            ComponentData::Food => todo_ser!("Food"),
            ComponentData::Consumable => todo_ser!("Consumable"),
            ComponentData::UseRemainder(v) => one!(22, v.as_ref()),
            ComponentData::UseCooldown => todo_ser!("UseCooldown"),
            ComponentData::DamageResistant => todo_ser!("DamageResistant"),
            ComponentData::Tool => todo_ser!("Tool"),
            ComponentData::Weapon => todo_ser!("Weapon"),
            ComponentData::Enchantable(v) => one!(27, v),
            ComponentData::Equippable => todo_ser!("Equippable"),
            ComponentData::Repairable => todo_ser!("Repairable"),
            ComponentData::Glider => unit!(30),
            ComponentData::TooltipStyle(v) => one!(31, v),
            ComponentData::DeathProtection => todo_ser!("DeathProtection"),
            ComponentData::BlocksAttacks => todo_ser!("BlocksAttacks"),
            ComponentData::StoredEnchantments => todo_ser!("StoredEnchantments"),
            ComponentData::DyedColor => todo_ser!("DyedColor"),
            ComponentData::MapColor(v) => one!(36, v),
            ComponentData::MapId(v) => one!(37, v),
            ComponentData::MapDecorations(_) => nbt_ser!("MapDecorations"),
            ComponentData::MapPostProcessing(v) => one!(39, v),
            ComponentData::ChargedProjectiles(slots) => {
                let mut t = serializer.serialize_tuple(2 + slots.len())?;
                t.serialize_element(&VarInt(40))?;
                t.serialize_element(&VarInt(slots.len() as i32))?;
                for s in slots {
                    t.serialize_element(s)?;
                }
                t.end()
            }
            ComponentData::BundleContents(slots) => {
                let mut t = serializer.serialize_tuple(2 + slots.len())?;
                t.serialize_element(&VarInt(41))?;
                t.serialize_element(&VarInt(slots.len() as i32))?;
                for s in slots {
                    t.serialize_element(s)?;
                }
                t.end()
            }
            ComponentData::PotionContents => todo_ser!("PotionContents"),
            ComponentData::PotionDurationScale(v) => one!(43, v),
            ComponentData::SuspiciousStewEffects => todo_ser!("SuspiciousStewEffects"),
            ComponentData::WritableBookContent => todo_ser!("WritableBookContent"),
            ComponentData::WrittenBookContent => todo_ser!("WrittenBookContent"),
            ComponentData::Trim => todo_ser!("Trim"),
            ComponentData::DebugStickState(_) => nbt_ser!("DebugStickState"),
            ComponentData::EntityData(_) => nbt_ser!("EntityData"),
            ComponentData::BucketEntityData(_) => nbt_ser!("BucketEntityData"),
            ComponentData::BlockEntityData(_) => nbt_ser!("BlockEntityData"),
            ComponentData::Instrument => todo_ser!("Instrument"),
            ComponentData::ProvidesTrimMaterial => todo_ser!("ProvidesTrimMaterial"),
            ComponentData::OminousBottleAmplifier(v) => one!(54, v),
            ComponentData::JukeboxPlayable => todo_ser!("JukeboxPlayable"),
            ComponentData::ProvidesBannerPatterns(v) => one!(56, v),
            ComponentData::Recipes(_) => nbt_ser!("Recipes"),
            ComponentData::LodestoneTracker => todo_ser!("LodestoneTracker"),
            ComponentData::FireworkExplosion => todo_ser!("FireworkExplosion"),
            ComponentData::Fireworks => todo_ser!("Fireworks"),
            ComponentData::Profile => todo_ser!("Profile"),
            ComponentData::NoteBlockSound(v) => one!(62, v),
            ComponentData::BannerPatterns => todo_ser!("BannerPatterns"),
            ComponentData::BaseColor(v) => one!(64, v),
            ComponentData::PotDecorations(items) => {
                let mut t = serializer.serialize_tuple(2 + items.len())?;
                t.serialize_element(&VarInt(65))?;
                t.serialize_element(&VarInt(items.len() as i32))?;
                for v in items {
                    t.serialize_element(v)?;
                }
                t.end()
            }
            ComponentData::Container(slots) => {
                let mut t = serializer.serialize_tuple(2 + slots.len())?;
                t.serialize_element(&VarInt(66))?;
                t.serialize_element(&VarInt(slots.len() as i32))?;
                for s in slots {
                    t.serialize_element(s)?;
                }
                t.end()
            }
            ComponentData::BlockState => todo_ser!("BlockState"),
            ComponentData::Bees => todo_ser!("Bees"),
            ComponentData::Lock(v) => one!(69, v),
            ComponentData::ContainerLoot => todo_ser!("ContainerLoot"),
            ComponentData::BreakSound => todo_ser!("BreakSound"),
            ComponentData::VillagerVariant(v) => one!(72, v),
            ComponentData::WolfVariant => todo_ser!("WolfVariant"),
            ComponentData::WolfSoundVariant => todo_ser!("WolfSoundVariant"),
            ComponentData::WolfCollar(v) => one!(75, v),
            ComponentData::FoxVariant(v) => one!(76, v),
            ComponentData::SalmonSize(v) => one!(77, v),
            ComponentData::ParrotVariant(v) => one!(78, v),
            ComponentData::TropicalFishPattern(v) => one!(79, v),
            ComponentData::TropicalFishBaseColor(v) => one!(80, v),
            ComponentData::TropicalFishPatternColor(v) => one!(81, v),
            ComponentData::MooshroomVariant(v) => one!(82, v),
            ComponentData::RabbitVariant(v) => one!(83, v),
            ComponentData::PigVariant => todo_ser!("PigVariant"),
            ComponentData::CowVariant => todo_ser!("CowVariant"),
            ComponentData::ChickenVariant => todo_ser!("ChickenVariant"),
            ComponentData::FrogVariant => todo_ser!("FrogVariant"),
            ComponentData::HorseVariant(v) => one!(88, v),
            ComponentData::PaintingVariant => todo_ser!("PaintingVariant"),
            ComponentData::LlamaVariant(v) => one!(90, v),
            ComponentData::AxolotlVariant(v) => one!(91, v),
            ComponentData::CatVariant => todo_ser!("CatVariant"),
            ComponentData::CatCollar(v) => one!(93, v),
            ComponentData::SheepColor(v) => one!(94, v),
            ComponentData::ShulkerColor(v) => one!(95, v),
        }
    }
}
