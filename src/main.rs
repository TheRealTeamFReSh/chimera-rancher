use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod animals;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(animals::AnimalsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_physics)
        .add_startup_system(setup_boundaries)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = [0.0, 0.0].into();
}

//spawning map boundaries
fn setup_boundaries(mut commands: Commands) {
    /*Bottom Edge*/
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 20.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -250.0,0.0)));

    /*Top Edge*/
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 20.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 250.0, 0.0)));

    /*Left Edge*/
    commands
        .spawn()
        .insert(Collider::cuboid(20.0, 1000.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(-500.0, 0.0, 0.0)));

    /*Right Edge*/
    commands
        .spawn()
        .insert(Collider::cuboid(20.0, 1000.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(500.0, 0.0, 0.0)));
    
}
