use bevy::prelude::*;

mod animals;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(animals::AnimalsPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
