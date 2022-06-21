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

fn spawn_test_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_animal(
        &AnimalKind::Pig,
        Vec2::new(100.0, 50.0),
        &mut commands,
        &asset_server,
    );
}

pub fn spawn_animal(
    animal_kind: &AnimalKind,
    position: Vec2,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    match animal_kind {
        AnimalKind::Pig => spawn_pig(position, commands, asset_server),
        AnimalKind::Cow => todo!(),
        AnimalKind::Dog => todo!(),
        AnimalKind::Horse => todo!(),
        AnimalKind::Chicken => todo!(),
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
        .insert(Collider::cuboid(25.0, 10.0));
}

#[derive(Component)]
pub struct AnimalComponent {
    behavior: AnimalBehavior,
    stats: AnimalStats,
}

#[derive(Clone, Copy)]
pub struct AnimalStats {
    speed: f32,
    accel: f32,
    decel: f32,
}
