use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand_distr::{Distribution, UnitCircle};
use std::collections::HashMap;

use self::behavior::AnimalBehavior;

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
                decel: 7.0,
                collider_size: Vec2::new(25.0, 10.0),
                texture: "pig.png".to_string(),
                behavior: AnimalBehavior::Idle {
                    timer: Timer::from_seconds(2.0, false),
                    base_duration: 2.5,
                    duration_spread: 1.0,
                    direction: Vec2::default(),
                    is_moving: false,
                },
            },
        );
        animal_attr_res.insert(
            AnimalKind::Cow,
            AnimalAttributes {
                speed: 50.0,
                accel: 1.75,
                decel: 7.0,
                collider_size: Vec2::new(25.0, 10.0),
                texture: "cow.png".to_string(),
                behavior: AnimalBehavior::Idle {
                    timer: Timer::from_seconds(2.0, false),
                    base_duration: 3.5,
                    duration_spread: 0.5,
                    direction: Vec2::default(),
                    is_moving: false,
                },
            },
        );
        animal_attr_res.insert(
            AnimalKind::Dog,
            AnimalAttributes {
                speed: 80.0,
                accel: 2.2,
                decel: 7.0,
                collider_size: Vec2::new(25.0, 10.0),
                texture: "dog.png".to_string(),
                behavior: AnimalBehavior::Idle {
                    timer: Timer::from_seconds(2.0, false),
                    base_duration: 1.5,
                    duration_spread: 1.0,
                    direction: Vec2::default(),
                    is_moving: false,
                },
            },
        );
        animal_attr_res.insert(
            AnimalKind::Chicken,
            AnimalAttributes {
                speed: 70.0,
                accel: 2.0,
                decel: 7.0,
                collider_size: Vec2::new(12.0, 10.0),
                texture: "chicken.png".to_string(),
                behavior: AnimalBehavior::Idle {
                    timer: Timer::from_seconds(2.0, false),
                    base_duration: 1.0,
                    duration_spread: 0.9,
                    direction: Vec2::default(),
                    is_moving: false,
                },
            },
        );
        animal_attr_res.insert(
            AnimalKind::Horse,
            AnimalAttributes {
                speed: 100.0,
                accel: 3.0,
                decel: 7.0,
                collider_size: Vec2::new(25.0, 10.0),
                texture: "horse.png".to_string(),
                behavior: AnimalBehavior::Idle {
                    timer: Timer::from_seconds(2.0, false),
                    base_duration: 6.0,
                    duration_spread: 2.0,
                    direction: Vec2::default(),
                    is_moving: false,
                },
            },
        );

        app.insert_resource(animal_attr_res)
            .add_startup_system(spawn_test_system)
            .add_system(behavior::animal_behavior_system);
    }
}

#[derive(Hash, PartialEq, Eq)]
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
    behavior: AnimalBehavior,
    stats: AnimalStats,
}

// Stores stats for animals
#[derive(Clone, Copy)]
pub struct AnimalStats {
    speed: f32,
    accel: f32,
    decel: f32,
}

pub struct AnimalAttributes {
    speed: f32,
    accel: f32,
    decel: f32,
    collider_size: Vec2,
    texture: String,
    behavior: AnimalBehavior,
}

type AnimalAttributesResource = HashMap<AnimalKind, AnimalAttributes>;

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
        Vec2::new(0.0, 0.0),
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

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(&attributes.texture),
            ..default()
        })
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(AnimalComponent {
            behavior: attributes.behavior.clone(),
            stats: AnimalStats {
                speed: attributes.speed,
                accel: attributes.accel,
                decel: attributes.decel,
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            attributes.collider_size.x,
            attributes.collider_size.y,
        ))
        .insert(LockedAxes::ROTATION_LOCKED);
}
