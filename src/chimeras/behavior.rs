use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{ChimeraComponent, ChimeraSprite};
use crate::behaviors::UnitBehavior;
use crate::constants::{CHIMERA_IDLE_DURATION, CHIMERA_IDLE_DURATION_SPREAD};
use crate::player::Player;
use crate::villagers::VillagerComponent;
use crate::{behaviors, constants};

// Handles animals behaving according to their current behavior
pub fn chimera_behavior_system(
    time: Res<Time>,
    mut chimera_query: Query<(&mut ChimeraComponent, &mut Velocity, &Transform, &Children)>,
    mut sprite_query: Query<&mut Sprite, With<ChimeraSprite>>,
    player_query: Query<&Transform, With<Player>>,
    villager_query: Query<&Transform, With<VillagerComponent>>,
) {
    for (mut chimera, mut vel, transform, children) in chimera_query.iter_mut() {
        let sprite_entities = children.iter().take(2).copied().collect::<Vec<Entity>>();

        let mut sprites: [Mut<Sprite>; 2] = sprite_query
            .get_many_mut((*sprite_entities).try_into().unwrap())
            .unwrap();

        let (sprite_1, sprite_2) = sprites.split_at_mut(1);

        let stats = chimera.stats;

        let player_transform = player_query.iter().next().unwrap();

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
            } else if position.distance(test_pos) < constants::CHIMERA_PURSUE_RANGE {
                pursue_villager_pos = Some(test_pos);
            }
        }

        match &mut chimera.behavior {
            UnitBehavior::Idle {
                timer,
                base_duration,
                duration_spread,
                direction,
                is_moving,
            } => {
                behaviors::idle_behavior(
                    &mut vel,
                    vec![&mut sprite_1[0], &mut sprite_2[0]],
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
                } else if position.distance(player_position) < constants::CHIMERA_FOLLOW_RANGE {
                    chimera.behavior = behaviors::UnitBehavior::Follow {
                        target: Some(player_position),
                        distance: constants::CHIMERA_FOLLOW_DISTANCE,
                    }
                }
            }
            UnitBehavior::Pursue { target } => {
                behaviors::pursue_behavior(
                    &mut vel,
                    vec![&mut sprite_1[0], &mut sprite_2[0]],
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
                        timer: Timer::from_seconds(constants::CHIMERA_IDLE_DURATION, false),
                        base_duration: CHIMERA_IDLE_DURATION,
                        duration_spread: CHIMERA_IDLE_DURATION_SPREAD,
                        direction: Vec2::default(),
                        is_moving: false,
                    }
                }
            }
            UnitBehavior::Follow { target, distance } => {
                behaviors::follow_behavior(
                    &mut vel,
                    vec![&mut sprite_1[0], &mut sprite_2[0]],
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
                } else if position.distance(player_position) > constants::CHIMERA_FOLLOW_RANGE {
                    chimera.behavior = behaviors::UnitBehavior::Idle {
                        timer: Timer::from_seconds(constants::CHIMERA_IDLE_DURATION, false),
                        base_duration: constants::CHIMERA_IDLE_DURATION,
                        duration_spread: constants::CHIMERA_IDLE_DURATION_SPREAD,
                        direction: Vec2::default(),
                        is_moving: false,
                    }
                }
            }
            UnitBehavior::RunAway { target } => todo!(),
        }
    }
}
