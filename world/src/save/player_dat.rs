use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerDat {
    /* Common to all entities */
    #[serde(flatten)]
    entity_tags: Entity,
    /* Common to all mobs */
    #[serde(flatten)]
    mob_tags: Mob,
    abilities: Abilities,
    data_version: i32, // Increases for each snapshot & release.
    dimension: String,
    // TODO: EnderItems, confusing.
    #[serde(rename = "enteredNeatherPosition")]
    entered_neather_position: Option<NeatherPosition>,
    food_exhaustion_level: f32,
    food_level: i32,
    food_saturation_level: f32,
    food_tick_timer: i32,
    // TODO: Inventory
    last_death_location: Option<DeathLocation>,
    #[serde(rename = "playerGameType")]
    player_game_type: i32,
    previous_player_game_type: i32,
    // TODO: recipeBook
    // TODO: RootVehicle
    score: i32,
    seen_credits: bool,
    // Not stored in level.dat
    selected_item: Option<Item>,
    selected_item_slot: i32,
    shoulder_entity_left: Option<Entity>,
    shoulder_entity_right: Option<Entity>,
    sleep_timer: i16,
    spawn_dimension: String,
    spawn_forced: bool,
    spawn_x: Option<i32>,
    spawn_y: Option<i32>,
    spawn_z: Option<i32>,
    warden_spawn_tracker: WardenSpawnTracker,
    xp_level: i32,
    #[serde(rename = "XpP")]
    xp_percent: f32,
    xp_seed: i32,
    xp_total: i32,
}

/// Tags common between all entities.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Entity {
    air: i16,
    custom_name: String,
    custom_name_visible: bool,
    fall_distance: f32,
    fire: i16,
    glowing: bool,
    has_visual_fire: bool,
    id: String,
    invulnerable: bool,
    motion: [f64; 3], // Always contains 3 elements?
    no_gravity: bool,
    on_ground: bool,
    // TODO: Instead of using EntityTags here we should probably have some big enum of all entities.
    passengers: Vec<Entity>,
    portal_cooldown: i32,
    pos: [f64; 3],      // Always contains 3 elements?
    rotation: [f32; 2], // Always contains 2 elements?
    silent: bool,
    tags: Vec<String>,
    ticks_frozen: Option<i32>,
    uuid: [i32; 4], // TODO: Is really a UUID & always has length 4
}

/// Tags common between all entities.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Mob {
    // TODO: From wiki but "Tags for all mobs, except HandItems, ArmorItems, HandDropChances, ArmorDropChances, CanPickUpLoot, PersistenceRequired and Leash"
    // Unclear...
    absorption_amount: f32,
    active_effects: Option<Vec<PotionEffect>>,
    armor_drop_chances: [f32; 4], // Contains 4 elements for feet, legs, chest, head respectively.
    armor_items: [Item; 4], // List of items the mob is wearing in the order feet, legs, chest, head.
    attributes: Vec<Attribute>,
    brain: Brain,
    can_pick_up_loot: bool,
    death_loot_table: String,
    death_loot_table_seed: i64,
    death_time: i16,
    fall_flying: bool,
    health: f32,
    // Last time the mob was damaged in number of ticks since mob creation.
    hurt_by_timestamp: i32,
    // Number of ticks the mob turns red after being hit. 0 when not recently hit.
    hurt_time: i16,
    hand_drop_chances: [f32; 2], // Main hand and off hand respectively.
    hand_items: [Item; 2],       // Main hand and off hand respectively.
    leash: Option<Leash>,
    left_handed: bool,
    no_ai: bool,
    persistence_required: bool,
    sleeping_x: Option<i32>,
    sleeping_y: Option<i32>,
    sleeping_z: Option<i32>,
    // TODO: Team?
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PotionEffect {
    ambient: bool,
    amplifier: bool,
    duration: i32,
    // TODO: HiddenEffect?
    id: i32,
    show_icon: bool,
    show_particles: bool,
}

/// Tags common for all Items.
// TODO: tag, slot
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Item {
    count: i8,
    id: Option<String>, // If not specified => change to stone?
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Attribute {
    name: String,
    base: f64,
    modifiers: Vec<Modifier>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Modifier {
    amount: f64,
    name: String,
    operation: i32,
    uuid: [i32; 4], // TODO: UUID
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Brain {
    // TODO: Memories: "Empty for all but allays, axolotls, frogs, goats, piglins, villagers, and wardens"
    #[serde(rename = "memories")]
    memories: Memories,
}

#[derive(Debug, Deserialize)]
// TODO: Memories...
struct Memories {}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Leash {
    #[serde(rename_all = "PascalCase")]
    FencePost { i: i32, y: i32, z: i32 },
    #[serde(rename_all = "PascalCase")]
    UUID {
        uuid: [i32; 4], // TOOD: UUID
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Abilities {
    flying: bool,
    fly_speed: f32, // Always 0.05?
    instabuild: bool,
    invulnerable: bool,
    may_build: bool,
    may_fly: bool,
    walk_speed: f32, // Always 0.1
}

#[derive(Debug, Deserialize)]
struct NeatherPosition {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize)]
struct DeathLocation {
    dimnension: String,
    pos: [i32; 3],
}

#[derive(Debug, Deserialize)]
struct WardenSpawnTracker {
    cooldown_ticks: i32,
    ticks_since_last_warning: i32,
    warning_level: i32,
}
