use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::TweeningPlugin;
use states::GameStates;

mod animals;
mod animations;
mod behaviors;
mod camera;
mod chimeras;
mod constants;
mod health;
mod inventory_parts;
mod pause_menu;
mod player;
mod states;
mod stats_window;
mod villagers;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            resizable: false,
            height: 720.,
            width: 1280.,
            title: "Chimera Rancher - Rusty Jam #2".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(animals::AnimalsPlugin)
        .add_plugin(chimeras::ChimerasPlugin)
        .add_plugin(villagers::VillagersPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(animations::AnimationsPlugin)
        .add_plugin(stats_window::StatsWindowPlugin)
        .add_plugin(inventory_parts::InventoryUIPlugin)
        .add_plugin(pause_menu::PauseMenuPlugin)
        .add_plugin(TweeningPlugin)
        .add_state(GameStates::Game)
        .add_system_set(
            SystemSet::on_enter(GameStates::Game)
                .with_system(setup_physics)
                .with_system(setup_boundaries)
                .with_system(setup_areas),
        )
        .add_startup_system(constants::compute_max_stats)
        .run();
}

fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = [0.0, 0.0].into();
}
//FIGURE OUT WHY ITS NOT COMPLETELY EVEN LATER
//spawning map boundaries
fn setup_boundaries(mut commands: Commands) {
    /*Bottom Edge*/
    commands
        .spawn()
        .insert(Collider::cuboid(1000.0, 20.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0, -1500.0, 0.0,
        )));

    /*Top Edge*/
    commands
        .spawn()
        .insert(Collider::cuboid(1000.0, 20.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 1500.0, 0.0)));

    /*Left Edge*/
    commands
        .spawn()
        .insert(Collider::cuboid(20.0, 1500.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -1000.0, 0.0, 0.0,
        )));

    /*Right Edge*/
    commands
        .spawn()
        .insert(Collider::cuboid(20.0, 1500.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(1000.0, 0.0, 0.0)));

    //Right river
    commands
        .spawn()
        .insert(Collider::cuboid(450.0, 200.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(530.0, 0.0, 0.0)));

    //Left river
    commands
        .spawn()
        .insert(Collider::cuboid(450.0, 200.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(-530.0, 0.0, 0.0)));
}

fn setup_areas(mut commands: Commands) {
    //Human area
    commands
        .spawn()
        .insert(Collider::cuboid(980.0, 640.0))
        .insert(Sensor(true))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 840.0, 0.0)));

    //Evil area
    commands
        .spawn()
        .insert(Collider::cuboid(980.0, 640.0))
        .insert(Sensor(true))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -840.0, 0.0)));
}
