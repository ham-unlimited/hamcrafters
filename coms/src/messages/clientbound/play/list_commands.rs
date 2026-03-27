use crate::{
    McPacket,
    codec::{identifier::Identifier, prefixed_array::PrefixedArray, var_int::VarInt},
    messages::models::{
        game_mode::{GameMode, PreviousGameMode},
        position::Position,
    },
};
use mc_packet_macros::mc_packet;
use serde::{Deserialize, Serialize, de::{SeqAccess, Visitor}};

/// Clientbound list commands packet.
#[derive(Debug, Deserialize)]
#[mc_packet(0x10)]
pub struct ListCommands {
    /// The command tree, as a list of nodes.
    pub nodes: PrefixedArray<Node>,
    /// The index of the root node of the command tree.
    pub root_index: VarInt,
}

// TODO: Move somewhere else?
/// A node in the command tree.
#[derive(Debug)]
pub struct Node {
    /// Flags for this node.
    pub flags: NodeFlags,
    /// Indices of child nodes in the command tree.
    pub children: Vec<VarInt>,
    /// Index of the node this node redirects to, if any.
    pub redirect_node: Option<VarInt>,
    /// Name of this node. Present for literal and argument nodes.
    pub name: Option<String>,
    /// Parser for this node. Present for argument nodes only.
    pub parser: Option<Parser>,
    /// Suggestions type identifier for this node, if any.
    pub suggestions_type: Option<Identifier>,
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NodeVisitor;

        impl<'de> Visitor<'de> for NodeVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a command tree node")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let flags: NodeFlags = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("expected node flags"))?;

                let children_count: VarInt = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("expected children count"))?;

                let mut children = Vec::with_capacity(children_count.0 as usize);
                for _ in 0..children_count.0 {
                    let child: VarInt = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::custom("expected child node index"))?;
                    children.push(child);
                }

                let redirect_node = if flags.has_redirect {
                    Some(
                        seq.next_element::<VarInt>()?
                            .ok_or_else(|| serde::de::Error::custom("expected redirect node index"))?,
                    )
                } else {
                    None
                };

                let name = match flags.node_type {
                    NodeType::Literal | NodeType::Argument => Some(
                        seq.next_element::<String>()?
                            .ok_or_else(|| serde::de::Error::custom("expected node name"))?,
                    ),
                    NodeType::Root => None,
                };

                let parser = if matches!(flags.node_type, NodeType::Argument) {
                    let parser_id: VarInt = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::custom("expected parser ID"))?;
                    Some(read_parser_from_seq(parser_id.0, &mut seq)?)
                } else {
                    None
                };

                let suggestions_type = if flags.has_suggestions_type {
                    Some(
                        seq.next_element::<Identifier>()?
                            .ok_or_else(|| serde::de::Error::custom("expected suggestions type identifier"))?,
                    )
                } else {
                    None
                };

                Ok(Node {
                    flags,
                    children,
                    redirect_node,
                    name,
                    parser,
                    suggestions_type,
                })
            }
        }

        deserializer.deserialize_seq(NodeVisitor)
    }
}

fn read_parser_from_seq<'de, A: SeqAccess<'de>>(id: i32, seq: &mut A) -> Result<Parser, A::Error> {
    match id {
        0 => Ok(Parser::Bool),
        1 => {
            let flags: u8 = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected float parser flags"))?;
            let min = if flags & 0x01 != 0 {
                seq.next_element::<f32>()?
                    .ok_or_else(|| serde::de::Error::custom("expected float min"))?
            } else {
                -f32::MAX
            };
            let max = if flags & 0x02 != 0 {
                seq.next_element::<f32>()?
                    .ok_or_else(|| serde::de::Error::custom("expected float max"))?
            } else {
                f32::MAX
            };
            Ok(Parser::Float(FloatParser { min, max }))
        }
        2 => {
            let flags: u8 = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected double parser flags"))?;
            let min = if flags & 0x01 != 0 {
                seq.next_element::<f64>()?
                    .ok_or_else(|| serde::de::Error::custom("expected double min"))?
            } else {
                -f64::MAX
            };
            let max = if flags & 0x02 != 0 {
                seq.next_element::<f64>()?
                    .ok_or_else(|| serde::de::Error::custom("expected double max"))?
            } else {
                f64::MAX
            };
            Ok(Parser::Double(DoubleParser { min, max }))
        }
        3 => {
            let flags: u8 = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected integer parser flags"))?;
            let min = if flags & 0x01 != 0 {
                seq.next_element::<i32>()?
                    .ok_or_else(|| serde::de::Error::custom("expected integer min"))?
            } else {
                i32::MIN
            };
            let max = if flags & 0x02 != 0 {
                seq.next_element::<i32>()?
                    .ok_or_else(|| serde::de::Error::custom("expected integer max"))?
            } else {
                i32::MAX
            };
            Ok(Parser::Integer(IntegerParser { min, max }))
        }
        4 => {
            let flags: u8 = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected long parser flags"))?;
            let min = if flags & 0x01 != 0 {
                seq.next_element::<i64>()?
                    .ok_or_else(|| serde::de::Error::custom("expected long min"))?
            } else {
                i64::MIN
            };
            let max = if flags & 0x02 != 0 {
                seq.next_element::<i64>()?
                    .ok_or_else(|| serde::de::Error::custom("expected long max"))?
            } else {
                i64::MAX
            };
            Ok(Parser::Long(LongParser { min, max }))
        }
        5 => {
            let behavior: VarInt = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected string behavior"))?;
            let behavior = match behavior.0 {
                0 => StringBehavior::SingleWord,
                1 => StringBehavior::QuotablePhrase,
                2 => StringBehavior::GreedyPhrase,
                _ => return Err(serde::de::Error::custom(format!("unknown string behavior: {}", behavior.0))),
            };
            Ok(Parser::String(behavior))
        }
        6 => {
            let flags: u8 = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected entity parser flags"))?;
            Ok(Parser::Entity(EntityFlags {
                single_entity: flags & 0x01 != 0,
                players_only: flags & 0x02 != 0,
            }))
        }
        7 => Ok(Parser::GameProfile),
        8 => Ok(Parser::BlockPos),
        9 => Ok(Parser::ColumnPos),
        10 => Ok(Parser::Vec3),
        11 => Ok(Parser::Vec2),
        12 => Ok(Parser::BlockState),
        13 => Ok(Parser::BlockPredicate),
        14 => Ok(Parser::ItemStack),
        15 => Ok(Parser::ItemPredicate),
        16 => Ok(Parser::Color),
        17 => Ok(Parser::HexColor),
        18 => Ok(Parser::Component),
        19 => Ok(Parser::Style),
        20 => Ok(Parser::Message),
        21 => Ok(Parser::NbtCompoundTag),
        22 => Ok(Parser::NbtTag),
        23 => Ok(Parser::NbtPath),
        24 => Ok(Parser::Objective),
        25 => Ok(Parser::ObjectiveCriteria),
        26 => Ok(Parser::Operation),
        27 => Ok(Parser::Particle),
        28 => Ok(Parser::Angle),
        29 => Ok(Parser::Rotation),
        30 => Ok(Parser::ScoreboardSlot),
        31 => {
            let flags: u8 = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected score_holder parser flags"))?;
            Ok(Parser::ScoreHolder(ScoreHolderFlags {
                allows_multiple: flags & 0x01 != 0,
            }))
        }
        32 => Ok(Parser::Swizzle),
        33 => Ok(Parser::Team),
        34 => Ok(Parser::ItemSlot),
        35 => Ok(Parser::ItemSlots),
        36 => Ok(Parser::ResourceLocation),
        37 => Ok(Parser::Function),
        38 => Ok(Parser::EntityAnchor),
        39 => Ok(Parser::IntRange),
        40 => Ok(Parser::FloatRange),
        41 => Ok(Parser::Dimension),
        42 => Ok(Parser::Gamemode),
        43 => {
            let min: i32 = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected time min"))?;
            Ok(Parser::Time(TimeParser { min }))
        }
        44 => {
            let registry: Identifier = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected resource_or_tag registry"))?;
            Ok(Parser::ResourceOrTag(registry))
        }
        45 => {
            let registry: Identifier = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected resource_or_tag_key registry"))?;
            Ok(Parser::ResourceOrTagKey(registry))
        }
        46 => {
            let registry: Identifier = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected resource registry"))?;
            Ok(Parser::Resource(registry))
        }
        47 => {
            let registry: Identifier = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected resource_key registry"))?;
            Ok(Parser::ResourceKey(registry))
        }
        48 => {
            let registry: Identifier = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::custom("expected resource_selector registry"))?;
            Ok(Parser::ResourceSelector(registry))
        }
        49 => Ok(Parser::TemplateMirror),
        50 => Ok(Parser::TemplateRotation),
        51 => Ok(Parser::Heightmap),
        52 => Ok(Parser::LootTable),
        53 => Ok(Parser::LootPredicate),
        54 => Ok(Parser::LootModifier),
        55 => Ok(Parser::Dialog),
        56 => Ok(Parser::Uuid),
        _ => Err(serde::de::Error::custom(format!("unknown parser ID: {id}"))),
    }
}

/// Flags for a command node, encoded as a bitfield in the protocol.
#[derive(Debug)]
pub struct NodeFlags {
    /// The type of node this is.
    pub node_type: NodeType,
    /// Whether this node is executable or not.
    pub is_executable: bool,
    /// Whether this node redirects to another node or not.
    pub has_redirect: bool,
    /// Whether this node has a suggestions type or not.
    pub has_suggestions_type: bool,
    /// Whether this requires special permissions. (OP)
    pub is_restricted: bool,
}

impl<'de> Deserialize<'de> for NodeFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let flags = u8::deserialize(deserializer)?;

        let node_type = match flags & 0x3 {
            0 => NodeType::Root,
            1 => NodeType::Literal,
            2 => NodeType::Argument,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid node type: {}",
                    flags & 0x3
                )));
            }
        };

        Ok(NodeFlags {
            node_type,
            is_executable: flags & 0x4 != 0,
            has_redirect: flags & 0x8 != 0,
            has_suggestions_type: flags & 0x10 != 0,
            is_restricted: flags & 0x20 != 0,
        })
    }
}

impl Serialize for NodeFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut flags = match self.node_type {
            NodeType::Root => 0,
            NodeType::Literal => 1,
            NodeType::Argument => 2,
        };

        if self.is_executable {
            flags |= 0x4;
        }
        if self.has_redirect {
            flags |= 0x8;
        }
        if self.has_suggestions_type {
            flags |= 0x10;
        }
        if self.is_restricted {
            flags |= 0x20;
        }

        serializer.serialize_u8(flags)
    }
}

/// The type of a command node.
#[derive(Debug)]
pub enum NodeType {
    /// A literal node, which matches a specific string.
    Literal,
    /// An argument node, which matches a specific type of argument.
    Argument,
    /// A root node, which is the starting point of the command tree.
    Root,
}

/// The argument parser for a command node.
#[derive(Debug)]
pub enum Parser {
    /// `brigadier:bool` — a boolean value.
    Bool,
    /// `brigadier:float` — a float with optional min/max.
    Float(FloatParser),
    /// `brigadier:double` — a double with optional min/max.
    Double(DoubleParser),
    /// `brigadier:integer` — an integer with optional min/max.
    Integer(IntegerParser),
    /// `brigadier:long` — a long with optional min/max.
    Long(LongParser),
    /// `brigadier:string` — a string with a parsing behavior.
    String(StringBehavior),
    /// `minecraft:entity` — a selector, player name, or UUID.
    Entity(EntityFlags),
    /// `minecraft:game_profile` — a player, online or not.
    GameProfile,
    /// `minecraft:block_pos` — integer 3D block position.
    BlockPos,
    /// `minecraft:column_pos` — integer 2D column position.
    ColumnPos,
    /// `minecraft:vec3` — floating-point 3D position.
    Vec3,
    /// `minecraft:vec2` — floating-point 2D position.
    Vec2,
    /// `minecraft:block_state` — a block state.
    BlockState,
    /// `minecraft:block_predicate` — a block or block tag.
    BlockPredicate,
    /// `minecraft:item_stack` — an item stack.
    ItemStack,
    /// `minecraft:item_predicate` — an item or item tag.
    ItemPredicate,
    /// `minecraft:color` — a chat color name.
    Color,
    /// `minecraft:hex_color` — an RGB color encoded as hex.
    HexColor,
    /// `minecraft:component` — a JSON text component.
    Component,
    /// `minecraft:style` — a JSON text component style object.
    Style,
    /// `minecraft:message` — a chat message, potentially with selectors.
    Message,
    /// `minecraft:nbt_compound_tag` — an NBT compound tag.
    NbtCompoundTag,
    /// `minecraft:nbt_tag` — a partial NBT tag.
    NbtTag,
    /// `minecraft:nbt_path` — a path within an NBT value.
    NbtPath,
    /// `minecraft:objective` — a scoreboard objective.
    Objective,
    /// `minecraft:objective_criteria` — a scoreboard criterion.
    ObjectiveCriteria,
    /// `minecraft:operation` — a scoreboard operator.
    Operation,
    /// `minecraft:particle` — a particle effect identifier.
    Particle,
    /// `minecraft:angle` — an angle.
    Angle,
    /// `minecraft:rotation` — a 2D rotation.
    Rotation,
    /// `minecraft:scoreboard_slot` — a scoreboard display slot.
    ScoreboardSlot,
    /// `minecraft:score_holder` — something that can join a team.
    ScoreHolder(ScoreHolderFlags),
    /// `minecraft:swizzle` — a collection of up to 3 axes.
    Swizzle,
    /// `minecraft:team` — a team name.
    Team,
    /// `minecraft:item_slot` — an inventory slot name.
    ItemSlot,
    /// `minecraft:item_slots` — multiple inventory slot names.
    ItemSlots,
    /// `minecraft:resource_location` — a resource identifier.
    ResourceLocation,
    /// `minecraft:function` — a function.
    Function,
    /// `minecraft:entity_anchor` — `feet` or `eyes`.
    EntityAnchor,
    /// `minecraft:int_range` — an integer range.
    IntRange,
    /// `minecraft:float_range` — a floating-point range.
    FloatRange,
    /// `minecraft:dimension` — a dimension identifier.
    Dimension,
    /// `minecraft:gamemode` — a game mode.
    Gamemode,
    /// `minecraft:time` — a time duration with minimum ticks.
    Time(TimeParser),
    /// `minecraft:resource_or_tag` — identifier or tag name for a registry.
    ResourceOrTag(Identifier),
    /// `minecraft:resource_or_tag_key` — identifier or tag name for a registry.
    ResourceOrTagKey(Identifier),
    /// `minecraft:resource` — an identifier for a registry.
    Resource(Identifier),
    /// `minecraft:resource_key` — an identifier for a registry.
    ResourceKey(Identifier),
    /// `minecraft:resource_selector` — an identifier for a registry.
    ResourceSelector(Identifier),
    /// `minecraft:template_mirror` — a structure template mirror type.
    TemplateMirror,
    /// `minecraft:template_rotation` — a structure template rotation type.
    TemplateRotation,
    /// `minecraft:heightmap` — a heightmap type.
    Heightmap,
    /// `minecraft:loot_table` — a loot table identifier.
    LootTable,
    /// `minecraft:loot_predicate` — a loot predicate identifier.
    LootPredicate,
    /// `minecraft:loot_modifier` — a loot modifier identifier.
    LootModifier,
    /// `minecraft:dialog` — a dialog registry identifier.
    Dialog,
    /// `minecraft:uuid` — a UUID value.
    Uuid,
}

/// Properties for the `brigadier:float` parser.
#[derive(Debug)]
pub struct FloatParser {
    /// Minimum allowed value. Defaults to `-f32::MAX`.
    pub min: f32,
    /// Maximum allowed value. Defaults to `f32::MAX`.
    pub max: f32,
}

/// Properties for the `brigadier:double` parser.
#[derive(Debug)]
pub struct DoubleParser {
    /// Minimum allowed value. Defaults to `-f64::MAX`.
    pub min: f64,
    /// Maximum allowed value. Defaults to `f64::MAX`.
    pub max: f64,
}

/// Properties for the `brigadier:integer` parser.
#[derive(Debug)]
pub struct IntegerParser {
    /// Minimum allowed value. Defaults to `i32::MIN`.
    pub min: i32,
    /// Maximum allowed value. Defaults to `i32::MAX`.
    pub max: i32,
}

/// Properties for the `brigadier:long` parser.
#[derive(Debug)]
pub struct LongParser {
    /// Minimum allowed value. Defaults to `i64::MIN`.
    pub min: i64,
    /// Maximum allowed value. Defaults to `i64::MAX`.
    pub max: i64,
}

/// Parsing behavior for the `brigadier:string` parser.
#[derive(Debug)]
pub enum StringBehavior {
    /// Reads a single unquoted word.
    SingleWord = 0,
    /// Reads a quoted phrase, or a single word if not quoted.
    QuotablePhrase = 1,
    /// Reads everything remaining after the cursor.
    GreedyPhrase = 2,
}

/// Flags for the `minecraft:entity` parser.
#[derive(Debug)]
pub struct EntityFlags {
    /// If set, only a single entity/player is allowed.
    pub single_entity: bool,
    /// If set, only players are allowed (no other entities).
    pub players_only: bool,
}

/// Flags for the `minecraft:score_holder` parser.
#[derive(Debug)]
pub struct ScoreHolderFlags {
    /// If set, multiple score holders are allowed.
    pub allows_multiple: bool,
}

/// Properties for the `minecraft:time` parser.
#[derive(Debug)]
pub struct TimeParser {
    /// Minimum duration in ticks.
    pub min: i32,
}
