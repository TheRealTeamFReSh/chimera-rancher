use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

mod behavior;

use crate::animations::BobbingAnim;
use crate::behaviors::UnitBehavior;

pub struct VillagersPlugin;

impl Plugin for VillagersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_test_villager_system)
            .add_system(behavior::villager_behavior_system);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VillagerStats {
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
    pub damage: f32,
}

#[derive(Component)]
pub struct VillagerComponent {
    pub behavior: UnitBehavior,
    pub stats: VillagerStats,
}

#[derive(Component)]
pub struct VillagerSprite;

fn spawn_test_villager_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_villager(Vec2::new(0.0, 500.0), &mut commands, &asset_server);
    spawn_villager(Vec2::new(30.0, 500.0), &mut commands, &asset_server);
    spawn_villager(Vec2::new(20.0, 400.0), &mut commands, &asset_server);
    spawn_villager(Vec2::new(-50.0, 350.0), &mut commands, &asset_server);
}

pub fn spawn_villager(position: Vec2, commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_translation(
            position.extend(0.0),
        )))
        .insert(Velocity::default())
        .insert(VillagerComponent {
            behavior: UnitBehavior::Idle {
                timer: Timer::from_seconds(2.0, false),
                base_duration: 6.0,
                duration_spread: 2.0,
                direction: Vec2::default(),
                is_moving: false,
            },
            stats: VillagerStats {
                speed: 50.0,
                accel: 3.0,
                decel: 6.0,
                damage: 5.0,
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(10.0, 15.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("villager_1.png"),
                    ..default()
                })
                .insert(VillagerSprite)
                .insert(BobbingAnim {
                    anim: rand::thread_rng().gen::<f32>() * 32.0,
                });
        });
}
