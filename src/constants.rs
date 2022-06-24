use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    animals::{AnimalAttributes, AnimalKind},
    health::Health,
};

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

// behaviors
pub const FOLLOW_RANGE: f32 = 450.0;
pub const FOLLOW_DISTANCE: f32 = 100.0;
pub const PURSUE_RANGE: f32 = 250.0;

// bobbing anim
pub const ANIMATION_SPEED_FACTOR: f32 = 0.2;
pub const ANIMATION_OFFSET_FACTOR: f32 = 4.0;

// stats
pub const STATS_DEVIATION: f32 = 0.5;
pub const DEFAULT_HEALTH: Health = Health {
    health: 0.,
    max_health: 0.,
};

// compute maximal values
#[derive(Default)]
pub struct MaxStats {
    pub accel: f32,
    pub decel: f32,
    pub attack: f32,
    pub speed: f32,
    pub health: f32,
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
    }

    commands.insert_resource(maxi);
}
