use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

mod behavior;

use crate::animations::BobbingAnim;
use crate::behaviors::UnitBehavior;
use crate::health::Health;
const STATS_DEVIATION: f32 = 0.2;

pub struct VillagersPlugin;

impl Plugin for VillagersPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_test_villager_system)
            .add_system(behavior::villager_behavior_system);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VillagerStats {
    pub attack: f32,
    pub health: f32,
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
    let villager_health = rand::thread_rng()
        .gen_range(120.0 * (1.0 - STATS_DEVIATION)..120.0 * (1.0 + STATS_DEVIATION));

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_translation(
            position.extend(1.0),
        )))
        .insert(Velocity::default())
        .insert(VillagerComponent {
            behavior: UnitBehavior::Pursue { target: None },
            stats: VillagerStats {
                health: villager_health,
                attack: rand::thread_rng()
                    .gen_range(10.0 * (1.0 - STATS_DEVIATION)..100.0 * (1.0 + STATS_DEVIATION)),
                speed: rand::thread_rng()
                    .gen_range(100.0 * (1.0 - STATS_DEVIATION)..100.0 * (1.0 + STATS_DEVIATION)),
                accel: rand::thread_rng()
                    .gen_range(2.0 * (1.0 - STATS_DEVIATION)..2.0 * (1.0 + STATS_DEVIATION)),
                decel: rand::thread_rng()
                    .gen_range(6.0 * (1.0 - STATS_DEVIATION)..6.0 * (1.0 + STATS_DEVIATION)),
                damage: 5.0,
            },
        })
        .insert(Health::new(villager_health))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(10.0, 15.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(
                        format!("villager_{}.png", rand::thread_rng().gen_range(1..=3)).as_str(),
                    ),
                    ..default()
                })
                .insert(VillagerSprite)
                .insert(BobbingAnim {
                    anim: rand::thread_rng().gen::<f32>() * 32.0,
                });
        });
}
