use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{AnimalComponent, AnimalSprite};
use crate::{
    behaviors::{self, UnitBehavior},
    constants,
    player::Player,
};

// Handles animals behaving according to their current behavior
pub fn animal_behavior_system(
    time: Res<Time>,
    mut animal_query: Query<(&mut AnimalComponent, &mut Velocity, &Transform, &Children)>,
    mut sprite_query: Query<&mut Sprite, With<AnimalSprite>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (mut animal, mut vel, transform, children) in animal_query.iter_mut() {
        let sprite_entity = children.iter().next().unwrap();

        let mut sprite = sprite_query.get_mut(*sprite_entity).unwrap();

        let animal_position = Vec2::new(transform.translation.x, transform.translation.y);
        let stats = animal.stats;
        let player_transform = player_query.iter().next().unwrap();
        let player_position = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );
        match &mut animal.behavior {
            UnitBehavior::Idle {
                timer,
                base_duration,
                duration_spread,
                direction,
                is_moving,
            } => {
                behaviors::idle_behavior(
                    &mut vel,
                    vec![&mut sprite],
                    &time,
                    timer,
                    base_duration,
                    duration_spread,
                    direction,
                    is_moving,
                    stats.into(),
                );
                if animal_position.distance(player_position) < constants::ANIMAL_RUNAWAY_RANGE {
                    animal.behavior = UnitBehavior::RunAway {
                        target: Some(player_position),
                    }
                }
            }
            UnitBehavior::Pursue { target: _ } => todo!(),
            UnitBehavior::Follow {
                target: _,
                distance: _,
            } => todo!(),
            UnitBehavior::RunAway { target } => {
                behaviors::run_away_behavior(
                    &mut vel,
                    vec![&mut sprite],
                    stats.into(),
                    animal_position,
                    *target,
                );

                if animal_position.distance(player_position) > constants::ANIMAL_IDLE_RANGE {
                    animal.behavior = UnitBehavior::Idle {
                        timer: Timer::from_seconds(constants::ANIMAL_IDLE_DURATION, false),
                        base_duration: constants::ANIMAL_IDLE_DURATION,
                        duration_spread: constants::ANIMAL_IDLE_DURATION_SPREAD,
                        direction: Vec2::default(),
                        is_moving: false,
                    };
                } else {
                    animal.behavior = UnitBehavior::RunAway {
                        target: Some(player_position),
                    }
                }
            }
        }
    }
}
