use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::TweeningPlugin;
use bevy_ecs_tilemap::prelude::*;

mod animals;
mod animations;
mod behaviors;
mod camera;
mod chimeras;
mod health;
mod player;
mod stats_window;
mod villagers;
mod helpers;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(animals::AnimalsPlugin)
        .add_plugin(chimeras::ChimerasPlugin)
        .add_plugin(villagers::VillagersPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(animations::AnimationsPlugin)
        .add_plugin(stats_window::StatsWindowPlugin)
        .add_plugin(TweeningPlugin)
        .add_startup_system(setup_tiles)
        .add_startup_system(setup_physics)
        .add_startup_system(setup_boundaries)
        .add_startup_system(setup_areas)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        .run();
}

fn setup_tiles(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    let grass_handle: bevy::prelude::Handle<Image> = asset_server.load("grass2.png");
    // Create map entity and component
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    //Create new layer builder with a layer entity
    let (mut layer_builder, _) = LayerBuilder::new(
            &mut commands,
            LayerSettings::new(
                    MapSize(16, 16),
                    ChunkSize(8,8),
                    TileSize(32.0, 32.0),
                    //still don't know what this does
                    TextureSize(32.0, 32.0),
                ),
                0_u16,
                0_u16,
        );

    layer_builder.set_all(TileBundle::default());

    //Build layer
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, grass_handle);

    //Keep track of layers internally
    map.add_layer(&mut commands, 0_u16, layer_entity);

    //Spawn map
    commands.entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-2048.0, -2048.0, 0.0))
        .insert(GlobalTransform::default());


}


fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = [0.0, 0.0].into();
}

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
