use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{AnimalComponent, AnimalSprite};
use crate::behaviors;

// Handles animals behaving according to their current behavior
pub fn animal_behavior_system(
    time: Res<Time>,
    mut animal_query: Query<(&mut AnimalComponent, &mut Velocity, &Transform)>,
    mut sprite_query: Query<&mut Sprite, With<AnimalSprite>>,
) {
    for (mut animal, mut vel, _transform) in animal_query.iter_mut() {
        for mut sprite in sprite_query.iter_mut() {
            let stats = animal.stats;
            match &mut animal.behavior {
                behaviors::UnitBehavior::Idle {
                    timer,
                    base_duration,
                    duration_spread,
                    direction,
                    is_moving,
                } => behaviors::idle_behavior(
                    &mut vel,
                    vec![&mut sprite],
                    &time,
                    timer,
                    base_duration,
                    duration_spread,
                    direction,
                    is_moving,
                    stats.into(),
                ),
                behaviors::UnitBehavior::Pursue { target: _ } => todo!(),
                behaviors::UnitBehavior::Follow {
                    target: _,
                    distance: _,
                } => todo!(),
            }
        }
    }
}
