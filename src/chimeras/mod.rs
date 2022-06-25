use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use self::behavior::chimera_behavior_system;
use crate::{
    animals::AnimalKind,
    animations::BobbingAnim,
    assets_manager::AssetsManager,
    behaviors::{self, UnitBehavior},
    constants,
    health::Health,
    inventory_parts::interaction::InventoryManagement,
    player::Player,
    sound_manager::SpawnChimeraAudioChannel,
    states::GameStates,
};

mod behavior;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum ChimeraPartKind {
    Head(AnimalKind),
    Tail(AnimalKind),
}

#[derive(Component)]
pub struct ChimeraComponent {
    pub behavior: UnitBehavior,
    pub damage_timer: Timer,
    pub stats: ChimeraStats,
    pub attack_timer: Timer,
}

#[derive(Debug, Clone, Copy)]
pub struct ChimeraStats {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
    pub health: f32,
    pub attack: f32,
    pub regen: f32,
    pub range: f32,
}

// used for passing data from animals to chimeras
#[derive(Debug, Clone, PartialEq)]
pub struct ChimeraPartAttributes {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
    pub health: f32,
    pub attack: f32,
    pub regen: f32,
    pub range: f32,
    pub collider_size: Vec2,
    pub texture: Handle<Image>,
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
                //.with_system(test_spawn_chimera_system)
                .with_system(chimera_behavior_system)
                .with_system(behaviors::chimera_attack_system),
        );
    }
}

// spawns a random chimera from 2 parts in the player's inventory
pub fn _test_spawn_chimera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    assets: Res<AssetsManager>,
    mut player_query: Query<(&mut Player, &Transform)>,
    spawn_audio: Res<AudioChannel<SpawnChimeraAudioChannel>>,
    mut inv_man: ResMut<InventoryManagement>,
) {
    let capture_input = keyboard_input.just_pressed(KeyCode::P);

    if capture_input {
        // if there are 2 items selected
        if let Some((_, part1)) = inv_man.target_1.selection.clone() {
            if let Some((_, part2)) = inv_man.target_2.selection.clone() {
                if let Some((mut player, player_transform)) = player_query.iter_mut().next() {
                    let part_1_idx = player
                        .inventory
                        .chimera_parts
                        .iter()
                        .position(|part| part == &part1)
                        .unwrap();

                    player.inventory.chimera_parts.remove(part_1_idx);

                    let part_2_idx = player
                        .inventory
                        .chimera_parts
                        .iter()
                        .position(|part| part == &part2)
                        .unwrap();

                    player.inventory.chimera_parts.remove(part_2_idx);

                    // reset inv_man
                    inv_man.reset();

                    // play audio
                    spawn_audio.set_playback_rate(rand::thread_rng().gen_range(0.7..1.8));
                    spawn_audio.play(assets.sound_spawn_chimera.clone());

                    spawn_chimera(
                        (part1, part2),
                        Vec2::new(
                            player_transform.translation.x,
                            player_transform.translation.y + 150.0,
                        ),
                        &mut commands,
                    )
                }
            }
        }
    }
}

// spawns a chimera from two chimera parts
pub fn spawn_chimera(
    chimera_parts: (ChimeraPartAttributes, ChimeraPartAttributes),
    position: Vec2,
    commands: &mut Commands,
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
    let chimera_regen = head_attributes.regen + tail_attributes.regen;

    // spawn the chimera
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(Transform::from_translation(
            position.extend(10.0),
        )))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(ChimeraComponent {
            damage_timer: Timer::from_seconds(constants::DAMAGE_RED_DURATION, true),
            attack_timer: Timer::from_seconds(constants::CHIMERA_ATTACK_RATE, true),
            behavior: UnitBehavior::Follow {
                target: None,
                distance: constants::CHIMERA_FOLLOW_DISTANCE,
            },
            stats: ChimeraStats {
                attack: head_attributes.attack + tail_attributes.attack,
                range: head_attributes.range + tail_attributes.range,
                speed: head_attributes.speed + tail_attributes.speed,
                accel: head_attributes.accel + tail_attributes.accel,
                decel: head_attributes.decel + tail_attributes.decel,
                health: chimera_health,
                regen: chimera_regen,
            },
        })
        .insert(Health::new(chimera_health, 1.0, 1.0))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            head_attributes.collider_size.x / 2.0 + tail_attributes.collider_size.x / 2.0,
            head_attributes.collider_size.y,
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .with_children(|parent| {
            let bobbing_anim_val = rand::thread_rng().gen::<f32>() * 32.0;

            parent
                .spawn_bundle(SpriteBundle {
                    texture: head_attributes.texture,
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
                    texture: tail_attributes.texture,
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
