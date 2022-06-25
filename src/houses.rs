use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::states::GameStates;

pub struct HousesPlugin;

impl Plugin for HousesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStates::Game).with_system(spawn_houses),
            );
    }

}

fn spawn_houses(mut commands: Commands, asset_server: Res<AssetServer>) {
    let house_handle_1 = asset_server.load("house_export.png");
    let house_handle_2 = asset_server.load("house_export.png");
    let house_handle_3 = asset_server.load("house_export.png");
    let house_handle_4 = asset_server.load("house_export.png");
    let house_handle_5 = asset_server.load("house_export.png");
    let house_handle_6 = asset_server.load("house_export.png");
    //house 1
    commands
        .spawn()
        .insert(Collider::cuboid(60.0, 60.0))
        .insert_bundle( SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-600.0, 575.0, 1.0),
            ..Default::default()
        },
        texture: house_handle_1,
        ..Default::default()
        });
    //house 2
   commands
    .spawn()
    .insert(Collider::cuboid(60.0, 60.0))
    .insert_bundle( SpriteBundle {
    transform: Transform {
        translation: Vec3::new(0.0, 575.0, 1.0),
        ..Default::default()
    },
    texture: house_handle_2,
    ..Default::default()
    }); 

    commands
        .spawn()
        .insert(Collider::cuboid(60.0, 60.0))
        .insert_bundle( SpriteBundle {
        transform: Transform {
            translation: Vec3::new(600.0, 575.0, 1.0),
            ..Default::default()
        },
        texture: house_handle_3,
        ..Default::default()
        });

    commands
        .spawn()
        .insert(Collider::cuboid(60.0, 60.0))
        .insert_bundle( SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-600.0, 1075.0, 1.0),
            ..Default::default()
        },
        texture: house_handle_4,
        ..Default::default()
        });

    commands
        .spawn()
        .insert(Collider::cuboid(60.0, 60.0))
        .insert_bundle( SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 1075.0, 1.0),
            ..Default::default()
        },
        texture: house_handle_5,
        ..Default::default()
        });

    commands
        .spawn()
        .insert(Collider::cuboid(60.0, 60.0))
        .insert_bundle( SpriteBundle {
        transform: Transform {
            translation: Vec3::new(600.0, 1075.0, 1.0),
            ..Default::default()
        },
        texture: house_handle_6,
        ..Default::default()
        });
}

