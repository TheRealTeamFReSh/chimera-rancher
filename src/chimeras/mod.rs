use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::SliceRandom;
use rand::Rng;

use self::behavior::chimera_behavior_system;
use crate::{
    animals::AnimalKind, animations::BobbingAnim, behaviors::UnitBehavior, constants,
    health::Health, player::Player, states::GameStates,
};

mod behavior;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum ChimeraPartKind {
    Head(AnimalKind),
    Tail(AnimalKind),
}

#[derive(Component)]
pub struct ChimeraComponent {
    behavior: UnitBehavior,
    pub stats: ChimeraStats,
}

#[derive(Debug, Clone, Copy)]
pub struct ChimeraStats {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
    pub health: f32,
    pub attack: f32,
}

// used for passing data from animals to chimeras
#[derive(Debug, Clone, PartialEq)]
pub struct ChimeraPartAttributes {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
    pub health: f32,
    pub attack: f32,
    pub collider_size: Vec2,
    pub texture: String,
    pub kind: ChimeraPartKind,
}

#[derive(Component)]
pub struct ChimeraSprite;

pub struct ChimerasPlugin;

impl Plugin for ChimerasPlugin {
    fn build(&self, app: &mut App) {
        // on update
        app.add_system_set(
            SystemSet::on_update(GameStates::Game)
                .with_system(test_spawn_chimera_system)
                .with_system(chimera_behavior_system),
        );
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
    let mut tail_attributes = chimera_parts.1;

    // swap head and tail if they would otherwise create a backwards chimera
    if matches!(head_attributes.kind, ChimeraPartKind::Tail(_))
        && matches!(tail_attributes.kind, ChimeraPartKind::Head(_))
    {
        let temp = head_attributes.clone();
        head_attributes = tail_attributes;
        tail_attributes = temp;
    }

    let chimera_health = head_attributes.health + tail_attributes.health;

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
            behavior: UnitBehavior::Follow {
                target: None,
                distance: constants::CHIMERA_FOLLOW_DISTANCE,
            },
            stats: ChimeraStats {
                attack: head_attributes.attack + tail_attributes.attack,
                speed: head_attributes.speed + tail_attributes.speed,
                accel: head_attributes.accel + tail_attributes.accel,
                decel: head_attributes.decel + tail_attributes.decel,
                health: chimera_health,
            },
        })
        .insert(Health::new(chimera_health))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            head_attributes.collider_size.x / 2.0 + tail_attributes.collider_size.x / 2.0,
            head_attributes.collider_size.y,
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|parent| {
            let bobbing_anim_val = rand::thread_rng().gen::<f32>() * 32.0;

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
                .insert(BobbingAnim {
                    anim: bobbing_anim_val,
                });

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
                .insert(BobbingAnim {
                    anim: bobbing_anim_val,
                });
        });
}
