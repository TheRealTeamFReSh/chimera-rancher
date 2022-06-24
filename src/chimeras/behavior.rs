use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{ChimeraComponent, ChimeraSprite};
use crate::behaviors;

// Handles animals behaving according to their current behavior
pub fn chimera_behavior_system(
    time: Res<Time>,
    mut chimera_query: Query<(&mut ChimeraComponent, &mut Velocity, &Transform)>,
    mut sprite_query: Query<&mut Sprite, With<ChimeraSprite>>,
) {
    for (mut chimera, mut vel, _transform) in chimera_query.iter_mut() {
        let stats = chimera.stats;
        let mut sprite_iter = sprite_query.iter_mut();
        let mut sprite_1 = sprite_iter.next().unwrap();
        let mut sprite_2 = sprite_iter.next().unwrap();
        match &mut chimera.behavior {
            behaviors::UnitBehavior::Idle {
                timer,
                base_duration,
                duration_spread,
                direction,
                is_moving,
            } => behaviors::idle_behavior(
                &mut vel,
                vec![&mut sprite_1, &mut sprite_2],
                &time,
                timer,
                base_duration,
                duration_spread,
                direction,
                is_moving,
                stats.into(),
            ),
            behaviors::UnitBehavior::Pursue { target } => todo!(),
        }
    }
}
