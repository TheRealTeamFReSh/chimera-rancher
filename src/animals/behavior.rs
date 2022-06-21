use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::AnimalComponent;

pub enum AnimalBehavior {
    Idle { timer: Timer, base_duration: f32 },
}

pub fn animal_behavior_system(mut animal_query: Query<(&AnimalComponent, &mut Velocity)>) {
    for (animal, mut vel) in animal_query.iter_mut() {
        match &animal.behavior {
            AnimalBehavior::Idle {
                timer,
                base_duration,
            } => animal_idle(timer, base_duration),
        }
    }
}

pub fn animal_idle(timer: &Timer, base_duration: &f32) {}
