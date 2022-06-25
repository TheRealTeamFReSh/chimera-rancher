use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{projectile, states::GameStates, villagers::VillagerComponent};

#[derive(Debug, Component)]
pub struct Projectile {
    pub accel: f32,
    pub speed: f32,
    pub target: Vec2,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        // on update
        app.add_system_set(
            SystemSet::on_update(GameStates::Game)
                .with_system(move_projectile)
                .with_system(handle_projectile_collision),
        );
    }
}

fn move_projectile(
    mut query: Query<(
        &Projectile,
        &mut Velocity,
        &mut Transform,
        &mut TextureAtlasSprite,
    )>,
) {
    for (projectile, mut vel, transform, _) in query.iter_mut() {
        let target = projectile.target;
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        let direction = (target - position).normalize();

        vel.linvel.x += projectile.accel * direction.x;
        vel.linvel.y += projectile.accel * direction.y;

        if vel.linvel.x.abs() > projectile.speed * direction.x.abs() {
            vel.linvel.x = projectile.speed * direction.x;
        }
        if vel.linvel.y.abs() > projectile.speed * direction.y.abs() {
            vel.linvel.y = projectile.speed * direction.y;
        }
    }
}

fn handle_projectile_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query_projectile: Query<(Entity, &Projectile, &Transform)>,
) {
    for collision_event in collision_events.iter() {
        for (entity, _projectile, p_transform) in query_projectile.iter_mut() {
            println!("Received villager collision event: {:?}", collision_event);
            // is it intersecting with a villager?
            // is it intersecting with a boundary?
        }
    }
}
