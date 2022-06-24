use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{ChimeraComponent, ChimeraSprite};
use crate::player::Player;
use crate::villagers::VillagerComponent;
use crate::{behaviors, constants};

// Handles animals behaving according to their current behavior
pub fn chimera_behavior_system(
    time: Res<Time>,
    mut chimera_query: Query<(&mut ChimeraComponent, &mut Velocity, &Transform)>,
    mut sprite_query: Query<&mut Sprite, With<ChimeraSprite>>,
    player_query: Query<&Transform, With<Player>>,
    villager_query: Query<&Transform, With<VillagerComponent>>,
) {
    for (mut chimera, mut vel, transform) in chimera_query.iter_mut() {
        let stats = chimera.stats;
        let player_transform = player_query.iter().next().unwrap();
        let mut sprite_iter = sprite_query.iter_mut();
        let mut sprite_1 = sprite_iter.next().unwrap();
        let mut sprite_2 = sprite_iter.next().unwrap();

        let position = Vec2::new(transform.translation.x, transform.translation.y);

        let player_position = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );

        let mut pursue_villager_pos = None;

        for villager_transform in villager_query.iter() {
            let test_pos = Vec2::new(
                villager_transform.translation.x,
                villager_transform.translation.y,
            );

            if let Some(lowest_pos) = pursue_villager_pos {
                if position.distance(test_pos) < position.distance(lowest_pos) {
                    pursue_villager_pos = Some(test_pos);
                }
            } else if position.distance(test_pos) < constants::PURSUE_RANGE {
                pursue_villager_pos = Some(test_pos);
            }
        }

        match &mut chimera.behavior {
            behaviors::UnitBehavior::Idle {
                timer,
                base_duration,
                duration_spread,
                direction,
                is_moving,
            } => {
                behaviors::idle_behavior(
                    &mut vel,
                    vec![&mut sprite_1, &mut sprite_2],
                    &time,
                    timer,
                    base_duration,
                    duration_spread,
                    direction,
                    is_moving,
                    stats.into(),
                );

                if let Some(villager_pos) = pursue_villager_pos {
                    chimera.behavior = behaviors::UnitBehavior::Pursue {
                        target: Some(villager_pos),
                    };
                } else if position.distance(player_position) < constants::FOLLOW_RANGE {
                    chimera.behavior = behaviors::UnitBehavior::Follow {
                        target: Some(player_position),
                        distance: constants::FOLLOW_DISTANCE,
                    }
                }
            }
            behaviors::UnitBehavior::Pursue { target } => {
                behaviors::pursue_behavior(
                    &mut vel,
                    vec![&mut sprite_1, &mut sprite_2],
                    stats.into(),
                    position,
                    *target,
                );

                if pursue_villager_pos.is_some() {
                    // target the player
                    chimera.behavior = behaviors::UnitBehavior::Pursue {
                        target: pursue_villager_pos,
                    }
                } else {
                    chimera.behavior = behaviors::UnitBehavior::Idle {
                        timer: Timer::from_seconds(2.0, false),
                        base_duration: 2.5,
                        duration_spread: 1.0,
                        direction: Vec2::default(),
                        is_moving: false,
                    }
                }
            }
            behaviors::UnitBehavior::Follow { target, distance } => {
                behaviors::follow_behavior(
                    &mut vel,
                    vec![&mut sprite_1, &mut sprite_2],
                    stats.into(),
                    position,
                    *target,
                    *distance,
                );

                // target the player

                chimera.behavior = behaviors::UnitBehavior::Follow {
                    target: Some(player_position),
                    distance: *distance,
                };

                if let Some(villager_pos) = pursue_villager_pos {
                    chimera.behavior = behaviors::UnitBehavior::Pursue {
                        target: Some(villager_pos),
                    };
                } else if position.distance(player_position) > constants::FOLLOW_RANGE {
                    chimera.behavior = behaviors::UnitBehavior::Idle {
                        timer: Timer::from_seconds(2.0, false),
                        base_duration: 2.5,
                        duration_spread: 1.0,
                        direction: Vec2::default(),
                        is_moving: false,
                    }
                }
            }
        }
    }
}
