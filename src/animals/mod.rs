use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, UnitCircle};
use std::collections::HashMap;

use crate::animations::BobbingAnim;
use crate::behaviors::UnitBehavior;
use crate::constants;
use crate::health::Health;
use crate::states::GameStates;

mod behavior;

pub struct AnimalsPlugin;

impl Plugin for AnimalsPlugin {
    fn build(&self, app: &mut App) {
        let mut animal_attr_res = AnimalAttributesResource::default();
        animal_attr_res.insert(
            AnimalKind::Pig,
            AnimalAttributes {
                speed: 60.0,
                accel: 1.5,
                decel: 6.0,
                health: 120.0,
                attack: 10.0,
                regen: 1.0,
                collider_size: Vec2::new(20.0, 10.0),
                texture: "pig.png".to_string(),
                head_texture: "pighead.png".to_string(),
                tail_texture: "pigtail.png".to_string(),
            },
        );
        animal_attr_res.insert(
            AnimalKind::Cow,
            AnimalAttributes {
                speed: 50.0,
                accel: 1.75,
                decel: 6.0,
                attack: 8.0,
                health: 150.0,
                regen: 2.0,
                collider_size: Vec2::new(20.0, 10.0),
                texture: "cow.png".to_string(),
                head_texture: "cowhead.png".to_string(),
                tail_texture: "cowtail.png".to_string(),
            },
        );
        animal_attr_res.insert(
            AnimalKind::Dog,
            AnimalAttributes {
                speed: 80.0,
                accel: 2.2,
                decel: 6.0,
                attack: 15.0,
                health: 100.0,
                regen: 0.7,
                collider_size: Vec2::new(20.0, 10.0),
                texture: "dog.png".to_string(),
                head_texture: "doghead.png".to_string(),
                tail_texture: "dogtail.png".to_string(),
            },
        );
        animal_attr_res.insert(
            AnimalKind::Chicken,
            AnimalAttributes {
                speed: 70.0,
                accel: 2.0,
                decel: 6.0,
                health: 75.0,
                attack: 18.0,
                regen: 0.7,
                collider_size: Vec2::new(20.0, 10.0),
                texture: "chicken.png".to_string(),
                head_texture: "chickenhead.png".to_string(),
                tail_texture: "chickentail.png".to_string(),
            },
        );
        animal_attr_res.insert(
            AnimalKind::Horse,
            AnimalAttributes {
                speed: 100.0,
                accel: 3.0,
                decel: 6.0,
                health: 140.0,
                attack: 12.0,
                regen: 1.0,
                collider_size: Vec2::new(20.0, 10.0),
                texture: "horse.png".to_string(),
                head_texture: "horsehead.png".to_string(),
                tail_texture: "horsetail.png".to_string(),
            },
        );

        app.insert_resource(animal_attr_res);

        app.add_system_set(SystemSet::on_enter(GameStates::Game).with_system(spawn_test_system));
        app.add_system_set(
            SystemSet::on_update(GameStates::Game).with_system(behavior::animal_behavior_system),
        );
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AnimalKind {
    Pig,
    Cow,
    Dog,
    Horse,
    Chicken,
}

// Core component of animal
#[derive(Component)]
pub struct AnimalComponent {
    pub behavior: UnitBehavior,
    pub stats: AnimalStats,
}

#[derive(Component)]
pub struct AnimalSprite;

// Stores stats for animals
#[derive(Clone, Copy, Debug)]
pub struct AnimalStats {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
    pub kind: AnimalKind,
    pub health: f32,
    pub attack: f32,
    pub regen: f32,
}

pub struct AnimalAttributes {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
    pub attack: f32,
    pub health: f32,
    pub regen: f32,
    pub collider_size: Vec2,
    pub texture: String,
    pub head_texture: String,
    pub tail_texture: String,
}

pub type AnimalAttributesResource = HashMap<AnimalKind, AnimalAttributes>;

// Test function, spawns one of each animal
fn spawn_test_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    animal_attr_res: Res<AnimalAttributesResource>,
) {
    spawn_animal(
        &AnimalKind::Pig,
        Vec2::new(400.0, 50.0),
        &animal_attr_res,
        &mut commands,
        &asset_server,
    );

    spawn_animal(
        &AnimalKind::Cow,
        Vec2::new(-50.0, -200.0),
        &animal_attr_res,
        &mut commands,
        &asset_server,
    );

    spawn_animal(
        &AnimalKind::Dog,
        Vec2::new(-200.0, 200.0),
        &animal_attr_res,
        &mut commands,
        &asset_server,
    );

    spawn_animal(
        &AnimalKind::Horse,
        Vec2::new(0.0, -300.0),
        &animal_attr_res,
        &mut commands,
        &asset_server,
    );

    spawn_animal(
        &AnimalKind::Chicken,
        Vec2::new(200.0, 75.0),
        &animal_attr_res,
        &mut commands,
        &asset_server,
    );
}

// Spawn the indicated animal at the position
pub fn spawn_animal(
    animal_kind: &AnimalKind,
    position: Vec2,
    animal_attr_res: &AnimalAttributesResource,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    let attributes = &animal_attr_res[animal_kind];

    let mut random_direction = Vec2::new(0.0, 0.0);
    let dir: [f32; 2] = UnitCircle.sample(&mut rand::thread_rng());
    random_direction.x = dir[0];
    random_direction.y = dir[1];

    let animal_health = rand::thread_rng().gen_range(
        attributes.health * (1.0 - constants::STATS_DEVIATION)
            ..attributes.health * (1.0 + constants::STATS_DEVIATION),
    );

    let animal_regen = rand::thread_rng().gen_range(
        attributes.regen * (1.0 - constants::STATS_DEVIATION)
            ..attributes.regen * (1.0 + constants::STATS_DEVIATION),
    );

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_translation(
            position.extend(0.0),
        )))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(AnimalComponent {
            behavior: UnitBehavior::Idle {
                timer: Timer::from_seconds(constants::ANIMAL_IDLE_DURATION, false),
                base_duration: constants::ANIMAL_IDLE_DURATION,
                duration_spread: constants::ANIMAL_IDLE_DURATION_SPREAD,
                direction: Vec2::default(),
                is_moving: false,
            },
            stats: AnimalStats {
                attack: rand::thread_rng().gen_range(
                    attributes.attack * (1.0 - constants::STATS_DEVIATION)
                        ..attributes.attack * (1.0 + constants::STATS_DEVIATION),
                ),
                speed: rand::thread_rng().gen_range(
                    attributes.speed * (1.0 - constants::STATS_DEVIATION)
                        ..attributes.speed * (1.0 + constants::STATS_DEVIATION),
                ),
                accel: rand::thread_rng().gen_range(
                    attributes.accel * (1.0 - constants::STATS_DEVIATION)
                        ..attributes.accel * (1.0 + constants::STATS_DEVIATION),
                ),
                decel: rand::thread_rng().gen_range(
                    attributes.decel * (1.0 - constants::STATS_DEVIATION)
                        ..attributes.decel * (1.0 + constants::STATS_DEVIATION),
                ),
                health: animal_health,
                regen: animal_regen,
                kind: *animal_kind,
            },
        })
        .insert(Health::new(
            animal_health,
            animal_regen,
            constants::ANIMAL_REGEN_RATE,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            attributes.collider_size.x,
            attributes.collider_size.y,
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(&attributes.texture),
                    ..default()
                })
                .insert(AnimalSprite)
                .insert(BobbingAnim {
                    anim: rand::thread_rng().gen::<f32>() * 32.0,
                });
        });
}
