use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use self::behavior::AnimalBehavior;

mod behavior;

pub struct AnimalsPlugin;

impl Plugin for AnimalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_test_system);
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
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("pig.png"),
            ..default()
        })
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(AnimalComponent {
            behavior: AnimalBehavior::Idle,
            speed: 3.0,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(25.0, 10.0));
}

#[derive(Component)]
pub struct AnimalComponent {
    behavior: AnimalBehavior,
    speed: f32,
}
