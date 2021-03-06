use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{VillagerComponent, VillagerSprite};
use crate::behaviors::{self, UnitBehavior};
use crate::player::Player;

// Handles animals behaving according to their current behavior
pub fn villager_behavior_system(
    time: Res<Time>,
    mut villager_query: Query<(&mut VillagerComponent, &mut Velocity, &Transform, &Children)>,
    mut sprite_query: Query<&mut Sprite, With<VillagerSprite>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (mut villager, mut vel, transform, children) in villager_query.iter_mut() {
        let sprite_entity = children.iter().next().unwrap();
        let mut sprite = sprite_query.get_mut(*sprite_entity).unwrap();

        if sprite.color.r() > 1.0 {
            villager.damage_timer.tick(time.delta());
            if villager.damage_timer.just_finished() {
                sprite.color.set_r(1.0);
            }
        }

        let player_transform = player_query.iter().next().unwrap();
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
            behaviors::UnitBehavior::Pursue { target } => {
                behaviors::pursue_behavior(
                    &mut vel,
                    vec![&mut sprite],
                    stats.into(),
                    Vec2::new(transform.translation.x, transform.translation.y),
                    *target,
                );

                // target the player
                villager.behavior = UnitBehavior::Pursue {
                    target: Some(Vec2::new(
                        player_transform.translation.x,
                        player_transform.translation.y,
                    )),
                }
            }
            UnitBehavior::Follow {
                target: _,
                distance: _,
            } => todo!(),
            UnitBehavior::RunAway { target: _ } => todo!(),
        }
    }
}
