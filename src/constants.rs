use std::collections::HashMap;

use bevy::prelude::*;

use crate::animals::{AnimalAttributes, AnimalKind};

// Zindexes
pub const Z_UI: f32 = 100.;
pub const Z_DAY_CYCLE: f32 = 50.;

// day-night cycle
pub const DAY_LENGTH: f32 = 60.0; // in seconds
pub const STARTING_HOUR: f32 = 20.0;
pub const MAX_ALPHA: f32 = 0.85;
pub const MIN_ALPHA: f32 = 0.1;
pub const NIGHT_HOURS: f32 = 8.0;

// stats splitting
pub const HEAD_SPEED_PERCENT: f32 = 0.35;
pub const TAIL_SPEED_PERCENT: f32 = 0.65;
pub const HEAD_ACCEL_PERCENT: f32 = 0.35;
pub const TAIL_ACCEL_PERCENT: f32 = 0.65;
pub const HEAD_DECEL_PERCENT: f32 = 0.35;
pub const TAIL_DECEL_PERCENT: f32 = 0.65;
pub const HEAD_HEALTH_PERCENT: f32 = 0.50;
pub const TAIL_HEALTH_PERCENT: f32 = 0.50;
pub const HEAD_ATTACK_PERCENT: f32 = 0.7;
pub const TAIL_ATTACK_PERCENT: f32 = 0.3;
pub const HEAD_REGEN_PERECENT: f32 = 0.5;
pub const TAIL_REGEN_PERECENT: f32 = 0.5;
pub const HEAD_RANGE_PERCENT: f32 = 0.6;
pub const TAIL_RANGE_PERCENT: f32 = 0.4;

// behaviors
pub const CHIMERA_FOLLOW_RANGE: f32 = 450.0;
pub const CHIMERA_FOLLOW_DISTANCE: f32 = 100.0;
pub const CHIMERA_PURSUE_RANGE: f32 = 250.0;
pub const ANIMAL_RUNAWAY_RANGE: f32 = 250.0;
pub const ANIMAL_IDLE_RANGE: f32 = 500.0;
pub const CHIMERA_IDLE_DURATION: f32 = 2.0;
pub const CHIMERA_IDLE_DURATION_SPREAD: f32 = 1.0;
pub const ANIMAL_IDLE_DURATION: f32 = 2.0;
pub const ANIMAL_IDLE_DURATION_SPREAD: f32 = 1.0;
pub const ANIMAL_REGEN_RATE: f32 = 2.0;
pub const VILLAGER_REGEN_RATE: f32 = 2.0;
pub const VILLAGER_ATTACK_RATE: f32 = 1.0;
pub const CHIMERA_ATTACK_RATE: f32 = 1.0;

// spawning
pub const ANIMAL_SPAWN_MAX_X: f32 = 900.0;
pub const ANIMAL_SPAWN_MIN_X: f32 = -900.0;
pub const ANIMAL_SPAWN_MAX_Y: f32 = 1400.0;
pub const ANIMAL_SPAWN_MIN_Y: f32 = 200.0;
pub const ANIMAL_BASE_SPAWN_DURATION: f32 = 6.0;
pub const ANIMAL_SPAWN_FACTOR: f32 = 25.0;
pub const ANIMAL_SPAWN_HOURS: (u8, u8) = (20, 6);
pub const VILLAGER_BASE_SPAWN_DURATION: f32 = 2.0;
pub const VILLAGER_SPAWN_FACTOR: f32 = 25.0;
pub const VILLAGER_SPAWN_HOURS: (u8, u8) = (6, 20);
pub const VILLAGER_SPAWN_LOCATIONS: [(f32, f32); 6] = [
    (-600.0, 500.0),
    (0.0, 500.0),
    (600.0, 500.0),
    (-600.0, 1000.0),
    (0.0, 1000.0),
    (600.0, 1000.0),
];

// bobbing anim
pub const ANIMATION_SPEED_FACTOR: f32 = 0.2;
pub const ANIMATION_OFFSET_FACTOR: f32 = 4.0;

// stats
pub const ANIMAL_STATS_DEVIATION: f32 = 0.5;
pub const VILLAGER_STATS_DEVIATION: f32 = 0.2;

// misc
pub const DAMAGE_RED_DURATION: f32 = 0.5;

// compute maximal values
#[derive(Default)]
pub struct MaxStats {
    pub accel: f32,
    pub decel: f32,
    pub attack: f32,
    pub speed: f32,
    pub health: f32,
    pub regen: f32,
    pub range: f32,
}

pub fn compute_max_stats(
    animal_attr: Res<HashMap<AnimalKind, AnimalAttributes>>,
    mut commands: Commands,
) {
    let mut maxi = MaxStats::default();

    for (_, attr) in animal_attr.iter() {
        maxi.accel = f32::max(maxi.accel, attr.accel * 1.5);
        maxi.decel = f32::max(maxi.decel, attr.decel * 1.5);
        maxi.attack = f32::max(maxi.attack, attr.attack * 1.5);
        maxi.speed = f32::max(maxi.speed, attr.speed * 1.5);
        maxi.health = f32::max(maxi.health, attr.health * 1.5);
        maxi.regen = f32::max(maxi.regen, attr.regen * 1.5);
        maxi.range = f32::max(maxi.range, attr.range * 1.5);
    }

    commands.insert_resource(maxi);
}
