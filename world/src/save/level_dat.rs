use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MinecraftLevelDat {
    #[serde(rename = "Data")]
    data: MinecraftLevelDatData,
}

#[derive(Serialize, Deserialize)]
struct MinecraftLevelDatData {
    #[serde(rename = "allowCommands")]
    allow_commands: Option<bool>,
    #[serde(rename = "BorderCenterX")]
    border_center_x: Option<f64>,
    #[serde(rename = "BorderCenterZ")]
    border_center_z: Option<f64>,
    #[serde(rename = "BorderDamagePerBlock")]
    border_damage_per_block: Option<f64>,
    #[serde(rename = "BorderSize")]
    border_size: Option<f64>,
    #[serde(rename = "BorderSafeZone")]
    border_safe_zone: Option<f64>,
    #[serde(rename = "BorderSizeLerpTarget")]
    border_size_lerp_target: Option<f64>,
    #[serde(rename = "BorderSizeLerpTime")]
    border_size_lerp_time: Option<i64>,
    #[serde(rename = "BorderWarninBlocks")]
    border_warning_blocks: Option<f64>,
    #[serde(rename = "BorderWarningTime")]
    border_warning_time: Option<f64>,
    #[serde(rename = "clearWeatherTime")]
    clear_weather_time: i32,
    // TODO: customBossEvents
    #[serde(rename = "DataPacks")]
    data_packs: DataPacks,
    #[serde(rename = "DataVersion")]
    data_version: i32,
    #[serde(rename = "DayTime")]
    day_time: i64,
    #[serde(rename = "Difficulty")]
    difficulty: i8,
    #[serde(rename = "DifficultyLocked")]
    difficulty_locked: bool,
    // TODO: DimensionData.
    // #[serde(rename = "DimensionData")]
    // dimension_data: DimensionData,
    #[serde(rename = "GameRules")]
    game_rules: GameRules,
    #[serde(rename = "WorldGenSettings")]
    world_gen_settings: WorldGenSettings,
    #[serde(rename = "GameType")]
    game_type: i32,
    hardcore: bool,
    initialized: bool,
    #[serde(rename = "LastPlayed")]
    last_played: i64, // TODO: Should be unix time in milliseconds, maybe spice up the type here.
    #[serde(rename = "LevelName")]
    level_name: String,
    #[serde(rename = "MapFeatures")]
    map_features: Option<bool>,
    #[serde(rename = "Player")]
    // Only present in SinglePlayer or if it already existed.
    player: Option<Player>,
    raining: bool,
    #[serde(rename = "rainTime")]
    rain_time: i32,
    #[serde(rename = "RandomSeed")]
    random_seed: Option<i64>,
    spawn: Spawn,
    // TODO: ScheduledEvents
    // TODO: Version
}

// #[derive(Deserialize)]
// struct CustomBossEvents {
//     id: BossId,
// }

// #[derive(Deserialize)]
// struct BossId {
//     players: Vec<Uuid>,
//     color: String,
// }

#[derive(Serialize, Deserialize)]
struct DataPacks {
    #[serde(rename = "Enabled")]
    enabled: Vec<String>,
    #[serde(rename = "Disabled")]
    disabled: Vec<String>,
}

// TODO: Whilst these supposedly have types, (bools / i32s), they are all stored as Strings... :zzz:
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GameRules {
    global_sound_events: String,
    tnt_explosion_drop_decay: String,
    ender_pearls_vanish_on_death: String,
    do_fire_tick: String,
    max_command_chain_length: String,
    spawner_blocks_enabled: String,
    do_vines_spread: String,
    disable_elytra_movement_check: String,
    lava_source_conversion: String,
    command_block_output: String,
    forgive_dead_players: String,
    players_nether_portal_creative_delay: String,
    do_mob_spawning: String,
    max_entity_cramming: String,
    tnt_explodes: String,
    allow_fire_ticks_away_from_player: String,
    locator_bar: String,
    universal_anger: String,
    players_sleeping_percentage: String,
    snow_accumulation_height: String,
    block_explosion_drop_decay: String,
    do_immediate_respawn: String,
    natural_regeneration: String,
    pvp: String,
    do_mob_loot: String,
    fall_damage: String,
    do_entity_drops: String,
    random_tick_speed: String,
    players_nether_portal_default_delay: String,
    spawn_radius: String,
    freeze_damage: String,
    command_blocks_enabled: String,
    send_command_feedback: String,
    do_warden_spawning: String,
    fire_damage: String,
    reduced_debug_info: String,
    water_source_conversion: String,
    projectiles_can_break_blocks: String,
    announce_advancements: String,
    drowning_damage: String,
    disable_raids: String,
    do_weather_cycle: String,
    mob_explosion_drop_decay: String,
    do_daylight_cycle: String,
    show_death_messages: String,
    do_tile_drops: String,
    spawn_monsters: String,
    allow_entering_nether_using_portals: String,
    do_insomnia: String,
    keep_inventory: String,
    disable_player_movement_check: String,
    do_limited_crafting: String,
    mob_griefing: String,
    command_modification_block_limit: String,
    do_trader_spawning: String,
    log_admin_commands: String,
    spectators_generate_chunks: String,
    do_patrol_spawning: String,
    max_command_fork_count: String,
}

#[derive(Serialize, Deserialize)]
struct WorldGenSettings {
    bonus_chest: bool,
    seed: i64,
    generate_features: bool,
    dimensions: Dimensions,
}

#[derive(Serialize, Deserialize)]
struct Dimensions {
    #[serde(rename = "minecraft:overworld")]
    overworld: Dimension,
    #[serde(rename = "minecraft:the_nether")]
    nether: Dimension,
    #[serde(rename = "minecraft:the_end")]
    end: Dimension,
}

#[derive(Serialize, Deserialize)]
struct Dimension {/* TODO */}

#[derive(Serialize, Deserialize)]
struct Player {/* TODO */}

#[derive(Serialize, Deserialize)]
struct Spawn {
    pos: Vec<i32>, // TODO: Position type, is represented as a size 3 vector of ints.
    pitch: f32,
    dimension: String,
    yaw: f32,
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use nbt::{
        read_nbt_file,
        ser::{deserializer::Deserializer, serializer::to_nbt_tag_type},
        snbt::Snbt,
    };
    use serde::Deserialize;

    use crate::save::level_dat::MinecraftLevelDat;

    #[test]
    fn test_deserialize_level_dat() {
        let Some(nbt) = read_nbt_file(Path::new("../test-data/test-world/level.dat"))
            .expect("Expect to read file correctly")
        else {
            panic!("Failed");
        };

        let deserializer = Deserializer::from_nbt_tag(nbt.payload);
        MinecraftLevelDat::deserialize(deserializer).expect("Failed to deserialize");
    }

    #[ignore = "Doesn't work atm because LevelDat struct isn't complete"]
    #[test]
    fn test_serialize_deserialize_level_dat() {
        let Some(nbt) = read_nbt_file(Path::new("../test-data/level.dat"))
            .expect("Expect to read file correctly")
        else {
            panic!("Failed");
        };

        let deserializer = Deserializer::from_nbt_tag(nbt.payload.clone());
        let dat = MinecraftLevelDat::deserialize(deserializer).expect("Failed to deserialize");
        let serialized = to_nbt_tag_type(&dat).expect("Failed to serialize level.dat");

        let ser: Snbt = (&serialized.unwrap()).into();
        let og: Snbt = (&nbt.payload).into();

        assert_eq!(ser, og)
    }
}
