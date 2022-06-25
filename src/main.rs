use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::TweeningPlugin;
use states::GameStates;

mod animals;
mod animations;
mod assets_manager;
mod behaviors;
mod camera;
mod chimeras;
mod constants;
mod day_cycle;
mod gameover;
mod health;
mod helpers;
mod houses;
mod hud;
mod inventory_parts;
mod main_menu;
mod pause_menu;
mod player;
mod projectile;
mod sound_manager;
mod spells;
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
        .insert_resource(ClearColor(Color::rgb_u8(168, 52, 235)))
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(assets_manager::AssetsManagerPlugin)
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
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(pause_menu::PauseMenuPlugin)
        .add_plugin(day_cycle::DayCyclePlugin)
        .add_plugin(health::HealthPlugin)
        .add_plugin(sound_manager::SoundChannelsPlugin)
        .add_plugin(spells::SpellsPlugin)
        .add_plugin(gameover::GameOverPlugin)
        .add_plugin(hud::HudPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(houses::HousesPlugin)
        .add_plugin(projectile::ProjectilePlugin)
        .add_state(GameStates::AssetsLoading)
        .add_system_set(
            SystemSet::on_enter(GameStates::Game)
                .after("setup_attributes")
                .with_system(constants::compute_max_stats)
                .with_system(setup_physics)
                .with_system(setup_boundaries)
                .with_system(setup_tiles)
 //               .with_system(setup_env_obj),
        )
        .add_system_set(
            SystemSet::on_update(GameStates::Game)
                .with_system(helpers::texture::set_texture_filters_to_nearest),
        )
        .run();
}
/*
fn setup_env_obj(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("GRASS+.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 25, 14);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 286,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)))
        .insert(Collider::cuboid(16.0, 16.0));
}
*/

fn setup_tiles(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    let grass_handle: bevy::prelude::Handle<Image> = asset_server.load("grass.png");
    // Create map entity and component
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0_u16, map_entity);

    //Create new layer builder with a layer entity
    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(16, 16),
            ChunkSize(8, 8),
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
    commands
        .entity(map_entity)
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

}

