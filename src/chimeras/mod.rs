use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::SliceRandom;

use self::behavior::chimera_behavior_system;
pub use self::chimera_part::ChimeraPartKind;
use crate::{behaviors::UnitBehavior, player::Player};

mod behavior;
mod chimera_part;

#[derive(Component)]
pub struct ChimeraComponent {
    behavior: UnitBehavior,
    stats: ChimeraStats,
}

#[derive(Debug, Clone, Copy)]
pub struct ChimeraStats {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
}

// used for passing data from animals to chimeras
#[derive(Debug, Clone, PartialEq)]
pub struct ChimeraPartAttributes {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
    pub collider_size: Vec2,
    pub texture: String,
    pub kind: ChimeraPartKind,
}

#[derive(Component)]
pub struct ChimeraSprite;

pub struct ChimerasPlugin;

impl Plugin for ChimerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(test_spawn_chimera_system)
            .add_system(chimera_behavior_system);
    }
}

// spawns a random chimera from 2 parts in the player's inventory
pub fn test_spawn_chimera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &Transform)>,
) {
    let capture_input = keyboard_input.just_pressed(KeyCode::P);

    if capture_input {
        if let Some((mut player, player_transform)) = player_query.iter_mut().next() {
            if player.inventory.chimera_parts.len() >= 2 {
                // pick 2 random chimera parts to combine
                let head_part = player
                    .inventory
                    .chimera_parts
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone();

                let head_part_idx = player
                    .inventory
                    .chimera_parts
                    .iter()
                    .position(|part| part == &head_part)
                    .unwrap();

                player.inventory.chimera_parts.remove(head_part_idx);

                let tail_part = player
                    .inventory
                    .chimera_parts
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone();

                let tail_part_idx = player
                    .inventory
                    .chimera_parts
                    .iter()
                    .position(|part| part == &tail_part)
                    .unwrap();

                player.inventory.chimera_parts.remove(tail_part_idx);

                spawn_chimera(
                    (head_part, tail_part),
                    Vec2::new(
                        player_transform.translation.x,
                        player_transform.translation.y + 150.0,
                    ),
                    &mut commands,
                    &asset_server,
                )
            }
        }
    }
}

// spawns a chimera from two chimera parts
pub fn spawn_chimera(
    chimera_parts: (ChimeraPartAttributes, ChimeraPartAttributes),
    position: Vec2,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    let mut head_attributes = chimera_parts.0.clone();
    let mut tail_attributes = chimera_parts.1.clone();

    // swap head and tail if they would otherwise create a backwards chimera
    if matches!(head_attributes.kind, ChimeraPartKind::Tail(_))
        && matches!(tail_attributes.kind, ChimeraPartKind::Head(_))
    {
        let temp = head_attributes.clone();
        head_attributes = tail_attributes;
        tail_attributes = temp;
    }

    // spawn the chimera
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(Transform::from_translation(
            position.extend(0.0),
        )))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(ChimeraComponent {
            behavior: UnitBehavior::Idle {
                timer: Timer::from_seconds(2.0, false),
                base_duration: 2.5,
                duration_spread: 1.0,
                direction: Vec2::default(),
                is_moving: false,
            },
            stats: ChimeraStats {
                speed: head_attributes.speed + tail_attributes.speed,
                accel: head_attributes.accel + tail_attributes.accel,
                decel: head_attributes.decel + tail_attributes.decel,
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            head_attributes.collider_size.x / 2.0 + tail_attributes.collider_size.x / 2.0,
            head_attributes.collider_size.y,
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&head_attributes.texture),
                    sprite: Sprite {
                        flip_x: matches!(head_attributes.kind, ChimeraPartKind::Tail(_)),
                        ..default()
                    },
                    ..default()
                })
                .insert(ChimeraSprite)
                .insert(Transform::from_xyz(
                    tail_attributes.collider_size.x - head_attributes.collider_size.x,
                    0.0,
                    0.0,
                ));

            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&tail_attributes.texture),
                    sprite: Sprite {
                        flip_x: matches!(tail_attributes.kind, ChimeraPartKind::Head(_)),
                        ..default()
                    },
                    ..default()
                })
                .insert(ChimeraSprite)
                .insert(Transform::from_xyz(
                    tail_attributes.collider_size.x - head_attributes.collider_size.x,
                    0.0,
                    0.0,
                ));
        });
}
