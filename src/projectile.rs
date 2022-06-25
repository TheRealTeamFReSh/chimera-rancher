use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    health::Health,
    states::GameStates,
    villagers::{VillagerComponent, VillagerSprite},
};

#[derive(Debug, Component)]
pub struct Projectile {
    pub despawn_timer: Timer,
    pub damage: f32,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        // on update
        app.add_system_set(
            SystemSet::on_update(GameStates::Game)
                .with_system(projectile_collision_system)
                .with_system(projectile_despawn_system),
        );
    }
}

fn projectile_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query_projectile: Query<(Entity, &mut Projectile)>,
    mut query_villagers: Query<(Entity, &mut VillagerComponent, &mut Health, &Children)>,
    mut villager_sprite_query: Query<&mut Sprite, With<VillagerSprite>>,
    time: Res<Time>,
) {
    for collision_event in collision_events.iter() {
        for (projectile_entity, mut projectile) in query_projectile.iter_mut() {
            for (villager_entity, mut villager, mut villager_health, children) in
                query_villagers.iter_mut()
            {
                if let CollisionEvent::Started(e_1, e_2, _) = collision_event {
                    if *e_1 == projectile_entity && *e_2 == villager_entity
                        || *e_2 == projectile_entity && *e_1 == villager_entity
                    {
                        info!("projetile collide with villager");
                        villager_health.health -= projectile.damage;
                        commands.entity(projectile_entity).despawn();
                        for &child in children.iter() {
                            if let Ok(mut villager_sprite) = villager_sprite_query.get_mut(child) {
                                villager_sprite.color.set_r(255.0);
                                villager.damage_timer.reset();
                            }
                        }
                    }
                }
            }
            // is it intersecting with a villager?
            // is it intersecting with a boundary?
        }
    }
}

fn projectile_despawn_system(
    mut commands: Commands,
    mut query_projectile: Query<(Entity, &mut Projectile, &Transform)>,
    time: Res<Time>,
) {
    for (projectile_entity, mut projectile, _) in query_projectile.iter_mut() {
        projectile.despawn_timer.tick(time.delta());
        if projectile.despawn_timer.just_finished() {
            commands.entity(projectile_entity).despawn();
        }
    }
}
