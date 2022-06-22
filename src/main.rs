use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod animals;
mod chimeras;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(animals::AnimalsPlugin)
        .add_plugin(chimeras::ChimerasPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = [0.0, 0.0].into();
}
