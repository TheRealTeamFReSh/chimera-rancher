use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{VillagerComponent, VillagerSprite};
use crate::behaviors;

// Handles animals behaving according to their current behavior
pub fn villager_behavior_system(
    time: Res<Time>,
    mut villager_query: Query<(&mut VillagerComponent, &mut Velocity, &Transform)>,
    mut sprite_query: Query<&mut Sprite, With<VillagerSprite>>,
) {
    for (mut villager, mut vel, _transform) in villager_query.iter_mut() {
        for mut sprite in sprite_query.iter_mut() {
            let stats = villager.stats;
            match &mut villager.behavior {
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
            }
        }
    }
}
