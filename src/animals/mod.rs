use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand_distr::{Distribution, UnitCircle};

use self::behavior::AnimalBehavior;

mod behavior;

pub struct AnimalsPlugin;

impl Plugin for AnimalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_test_system)
            .add_system(behavior::animal_behavior_system);
    }
}

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

// Test function, spawns one of each animal
fn spawn_test_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_animal(
        &AnimalKind::Pig,
        Vec2::new(400.0, 50.0),
        &mut commands,
        &asset_server,
    );

    spawn_animal(
        &AnimalKind::Cow,
        Vec2::new(-50.0, -200.0),
        &mut commands,
        &asset_server,
    );

    spawn_animal(
        &AnimalKind::Dog,
        Vec2::new(-200.0, 200.0),
        &mut commands,
        &asset_server,
    );

    spawn_animal(
        &AnimalKind::Horse,
        Vec2::new(0.0, -300.0),
        &mut commands,
        &asset_server,
    );

    spawn_animal(
        &AnimalKind::Chicken,
        Vec2::new(0.0, 0.0),
        &mut commands,
        &asset_server,
    );
}

// Spawn the indicated animal at the position
pub fn spawn_animal(
    animal_kind: &AnimalKind,
    position: Vec2,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    match animal_kind {
        AnimalKind::Pig => spawn_pig(position, commands, asset_server),
        AnimalKind::Cow => spawn_cow(position, commands, asset_server),
        AnimalKind::Dog => spawn_dog(position, commands, asset_server),
        AnimalKind::Horse => spawn_horse(position, commands, asset_server),
        AnimalKind::Chicken => spawn_chicken(position, commands, asset_server),
    }
}

fn spawn_pig(position: Vec2, commands: &mut Commands, asset_server: &AssetServer) {
    let mut random_direction = Vec2::new(0.0, 0.0);
    let dir: [f32; 2] = UnitCircle.sample(&mut rand::thread_rng());
    random_direction.x = dir[0];
    random_direction.y = dir[1];

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("pig.png"),
            ..default()
        })
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(AnimalComponent {
            behavior: AnimalBehavior::Idle {
                timer: Timer::from_seconds(2.0, false),
                base_duration: 2.5,
                duration_spread: 1.0,
                direction: random_direction,
                is_moving: false,
            },
            stats: AnimalStats {
                speed: 60.0,
                accel: 1.5,
                decel: 7.0,
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(25.0, 10.0))
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn spawn_cow(position: Vec2, commands: &mut Commands, asset_server: &AssetServer) {
    let mut random_direction = Vec2::new(0.0, 0.0);
    let dir: [f32; 2] = UnitCircle.sample(&mut rand::thread_rng());
    random_direction.x = dir[0];
    random_direction.y = dir[1];

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("cow.png"),
            ..default()
        })
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(AnimalComponent {
            behavior: AnimalBehavior::Idle {
                timer: Timer::from_seconds(2.0, false),
                base_duration: 3.5,
                duration_spread: 0.5,
                direction: random_direction,
                is_moving: false,
            },
            stats: AnimalStats {
                speed: 50.0,
                accel: 1.75,
                decel: 7.0,
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(25.0, 10.0))
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn spawn_dog(position: Vec2, commands: &mut Commands, asset_server: &AssetServer) {
    let mut random_direction = Vec2::new(0.0, 0.0);
    let dir: [f32; 2] = UnitCircle.sample(&mut rand::thread_rng());
    random_direction.x = dir[0];
    random_direction.y = dir[1];

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("dog.png"),
            ..default()
        })
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(AnimalComponent {
            behavior: AnimalBehavior::Idle {
                timer: Timer::from_seconds(2.0, false),
                base_duration: 1.5,
                duration_spread: 1.0,
                direction: random_direction,
                is_moving: false,
            },
            stats: AnimalStats {
                speed: 80.0,
                accel: 2.2,
                decel: 7.0,
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(25.0, 10.0))
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn spawn_horse(position: Vec2, commands: &mut Commands, asset_server: &AssetServer) {
    let mut random_direction = Vec2::new(0.0, 0.0);
    let dir: [f32; 2] = UnitCircle.sample(&mut rand::thread_rng());
    random_direction.x = dir[0];
    random_direction.y = dir[1];

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("horse.png"),
            ..default()
        })
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(AnimalComponent {
            behavior: AnimalBehavior::Idle {
                timer: Timer::from_seconds(2.0, false),
                base_duration: 6.0,
                duration_spread: 2.0,
                direction: random_direction,
                is_moving: false,
            },
            stats: AnimalStats {
                speed: 100.0,
                accel: 3.0,
                decel: 7.0,
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(25.0, 10.0))
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn spawn_chicken(position: Vec2, commands: &mut Commands, asset_server: &AssetServer) {
    let mut random_direction = Vec2::new(0.0, 0.0);
    let dir: [f32; 2] = UnitCircle.sample(&mut rand::thread_rng());
    random_direction.x = dir[0];
    random_direction.y = dir[1];

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("chicken.png"),
            ..default()
        })
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(AnimalComponent {
            behavior: AnimalBehavior::Idle {
                timer: Timer::from_seconds(2.0, false),
                base_duration: 1.0,
                duration_spread: 0.9,
                direction: random_direction,
                is_moving: false,
            },
            stats: AnimalStats {
                speed: 70.0,
                accel: 2.0,
                decel: 7.0,
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(12.0, 10.0))
        .insert(LockedAxes::ROTATION_LOCKED);
}
