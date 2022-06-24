use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{ChimeraComponent, ChimeraSprite};
use crate::behaviors;
use crate::player::Player;

// Handles animals behaving according to their current behavior
pub fn chimera_behavior_system(
    time: Res<Time>,
    mut chimera_query: Query<(&mut ChimeraComponent, &mut Velocity, &Transform)>,
    mut sprite_query: Query<&mut Sprite, With<ChimeraSprite>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (mut chimera, mut vel, transform) in chimera_query.iter_mut() {
        let stats = chimera.stats;
        let player_transform = player_query.iter().next().unwrap();
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
            behaviors::UnitBehavior::Follow { target, distance } => {
                behaviors::follow_behavior(
                    &mut vel,
                    vec![&mut sprite_1, &mut sprite_2],
                    stats.into(),
                    Vec2::new(transform.translation.x, transform.translation.y),
                    *target,
                    *distance,
                );

                // target the player
                chimera.behavior = behaviors::UnitBehavior::Follow {
                    target: Some(Vec2::new(
                        player_transform.translation.x,
                        player_transform.translation.y,
                    )),
                    distance: *distance,
                }
            }
        }
    }
}
