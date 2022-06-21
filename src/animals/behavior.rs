use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, UnitCircle};
use std::time::Duration;

use super::{AnimalComponent, AnimalStats};

const IDLE_DURATION_SPREAD: f32 = 2.0;
const ROUND_ZERO_RANGE: f32 = 10.0;

pub enum AnimalBehavior {
    Idle {
        timer: Timer,
        base_duration: f32,
        direction: Vec2,
        is_moving: bool,
    },
}

pub fn animal_behavior_system(
    time: Res<Time>,
    mut animal_query: Query<(&mut AnimalComponent, &mut Velocity, &mut Sprite)>,
) {
    for (mut animal, mut vel, mut sprite) in animal_query.iter_mut() {
        let stats = animal.stats;
        match &mut animal.behavior {
            AnimalBehavior::Idle {
                timer,
                base_duration,
                direction,
                is_moving,
            } => animal_idle(
                &mut vel,
                &mut sprite,
                &time,
                timer,
                base_duration,
                direction,
                is_moving,
                stats,
            ),
        }
    }
}

pub fn animal_idle(
    vel: &mut Velocity,
    sprite: &mut Sprite,
    time: &Time,
    timer: &mut Timer,
    base_duration: &f32,
    direction: &mut Vec2,
    is_moving: &mut bool,
    stats: AnimalStats,
) {
    timer.tick(time.delta());

    if timer.just_finished() {
        timer.set_duration(Duration::from_secs_f32(
            base_duration
                - rand::thread_rng().gen_range(-IDLE_DURATION_SPREAD..IDLE_DURATION_SPREAD),
        ));
        timer.reset();

        let dir: [f32; 2] = UnitCircle.sample(&mut rand::thread_rng());
        println!("{:?} is from the unit circle.", dir);

        direction.x = dir[0];
        direction.y = dir[1];

        sprite.flip_x = direction.x < 0.0;

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
}
