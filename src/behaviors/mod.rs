use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, UnitCircle};
use std::time::Duration;

use crate::animals::AnimalStats;
use crate::chimeras::ChimeraStats;
use crate::villagers::VillagerStats;

const ROUND_ZERO_RANGE: f32 = 10.0;

mod attack;

pub use self::attack::villager_attack_system;

pub struct UnitStats {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
}

impl From<AnimalStats> for UnitStats {
    fn from(stats: AnimalStats) -> Self {
        Self {
            speed: stats.speed,
            accel: stats.accel,
            decel: stats.decel,
        }
    }
}

impl From<ChimeraStats> for UnitStats {
    fn from(stats: ChimeraStats) -> Self {
        Self {
            speed: stats.speed,
            accel: stats.accel,
            decel: stats.decel,
        }
    }
}

impl From<VillagerStats> for UnitStats {
    fn from(stats: VillagerStats) -> Self {
        Self {
            speed: stats.speed,
            accel: stats.accel,
            decel: stats.decel,
        }
    }
}

// Enum that describes behaviors for animals
#[derive(Clone)]
pub enum UnitBehavior {
    Idle {
        timer: Timer,
        base_duration: f32,
        duration_spread: f32,
        direction: Vec2,
        is_moving: bool,
    },
    Pursue {
        target: Option<Vec2>,
    },
    Follow {
        target: Option<Vec2>,
        distance: f32,
    },
    RunAway {
        target: Option<Vec2>,
    },
}
// Handle animal idling behavior
pub fn idle_behavior(
    vel: &mut Velocity,
    sprites: Vec<&mut Sprite>,
    time: &Time,
    timer: &mut Timer,
    base_duration: &f32,
    duration_spread: &f32,
    direction: &mut Vec2,
    is_moving: &mut bool,
    stats: UnitStats,
) {
    timer.tick(time.delta());
    let old_linvel = vel.linvel;

    if timer.just_finished() {
        timer.set_duration(Duration::from_secs_f32(
            base_duration - rand::thread_rng().gen_range(-duration_spread..*duration_spread),
        ));
        timer.reset();

        let dir: [f32; 2] = UnitCircle.sample(&mut rand::thread_rng());

        direction.x = dir[0];
        direction.y = dir[1];

        *is_moving ^= true;
    }

    if *is_moving {
        if vel.linvel.x.abs() < (stats.speed * direction.x).abs() {
            vel.linvel.x += stats.accel * direction.x;
        }

        if vel.linvel.y.abs() < (stats.speed * direction.y).abs() {
            vel.linvel.y += stats.accel * direction.y;
        }
    } else if (vel.linvel.x.abs().powf(2.0) + vel.linvel.y.abs().powf(2.0)).sqrt()
        - ROUND_ZERO_RANGE
        < 0.0
    {
        vel.linvel.x = 0.0;
        vel.linvel.y = 0.0;
    } else {
        if vel.linvel.x > 0.0 {
            vel.linvel.x -= stats.decel * direction.x.abs();
        } else {
            vel.linvel.x += stats.decel * direction.x.abs();
        }
        if vel.linvel.y > 0.0 {
            vel.linvel.y -= stats.decel * direction.y.abs();
        } else {
            vel.linvel.y += stats.decel * direction.y.abs();
        }
    }

    if old_linvel.x.is_sign_positive() != vel.linvel.x.is_sign_positive() {
        for sprite in sprites {
            sprite.flip_x ^= true;
        }
    }
}

// Handle pursue behavior
pub fn pursue_behavior(
    vel: &mut Velocity,
    sprites: Vec<&mut Sprite>,
    stats: UnitStats,
    position: Vec2,
    target: Option<Vec2>,
) {
    if let Some(target) = target {
        let direction = (target - position).normalize();
        let old_linvel = vel.linvel;

        vel.linvel.x += stats.accel * direction.x;
        vel.linvel.y += stats.accel * direction.y;

        if vel.linvel.x.abs() > stats.speed * direction.x.abs() {
            vel.linvel.x = stats.speed * direction.x;
        }
        if vel.linvel.y.abs() > stats.speed * direction.y.abs() {
            vel.linvel.y = stats.speed * direction.y;
        }

        if old_linvel.x.is_sign_positive() != vel.linvel.x.is_sign_positive() {
            for sprite in sprites {
                sprite.flip_x ^= true;
            }
        }
    }
}

// Handle pursue behavior
pub fn follow_behavior(
    vel: &mut Velocity,
    sprites: Vec<&mut Sprite>,
    stats: UnitStats,
    position: Vec2,
    target: Option<Vec2>,
    distance: f32,
) {
    if let Some(target) = target {
        let direction = (target - position).normalize();

        let old_linvel = vel.linvel;

        if position.distance(target) > distance {
            vel.linvel.x += stats.accel * direction.x;
            vel.linvel.y += stats.accel * direction.y;
        } else if (vel.linvel.x.abs().powf(2.0) + vel.linvel.y.abs().powf(2.0)).sqrt()
            - ROUND_ZERO_RANGE
            < 0.0
        {
            vel.linvel.x = 0.0;
            vel.linvel.y = 0.0;
        } else {
            vel.linvel.x -= stats.decel * direction.x;
            vel.linvel.y -= stats.decel * direction.y;
        }

        if vel.linvel.x.abs() > stats.speed * direction.x.abs() {
            vel.linvel.x = stats.speed * direction.x;
        }
        if vel.linvel.y.abs() > stats.speed * direction.y.abs() {
            vel.linvel.y = stats.speed * direction.y;
        }

        if old_linvel.x.is_sign_positive() != vel.linvel.x.is_sign_positive() {
            for sprite in sprites {
                sprite.flip_x ^= true;
            }
        }
    }
}

// Handle pursue behavior
pub fn run_away_behavior(
    vel: &mut Velocity,
    sprites: Vec<&mut Sprite>,
    stats: UnitStats,
    position: Vec2,
    target: Option<Vec2>,
) {
    if let Some(target) = target {
        let direction = (position - target).normalize();
        let old_linvel = vel.linvel;

        vel.linvel.x += stats.accel * direction.x;
        vel.linvel.y += stats.accel * direction.y;

        if vel.linvel.x.abs() > stats.speed * direction.x.abs() {
            vel.linvel.x = stats.speed * direction.x;
        }
        if vel.linvel.y.abs() > stats.speed * direction.y.abs() {
            vel.linvel.y = stats.speed * direction.y;
        }

        if old_linvel.x.is_sign_positive() != vel.linvel.x.is_sign_positive() {
            for sprite in sprites {
                sprite.flip_x ^= true;
            }
        }
    }
}
