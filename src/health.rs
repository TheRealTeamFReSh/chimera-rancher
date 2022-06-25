use bevy::prelude::*;

use crate::{player::Player, states::GameStates};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        // on update
        app.add_system_set(SystemSet::on_update(GameStates::Game).with_system(health_system));
    }
}

#[derive(Component)]
pub struct Health {
    pub max_health: f32,
    pub health: f32,
    pub regen: f32,
    pub regen_timer: Timer,
}

impl Health {
    pub fn new(health: f32, regen: f32, regen_rate: f32) -> Self {
        Self {
            max_health: health,
            health,
            regen,
            regen_timer: Timer::from_seconds(regen_rate, true),
        }
    }
}

pub fn health_system(
    mut commands: Commands,
    mut health_query: Query<(Entity, &mut Health)>,
    player_query: Query<Entity, With<Player>>,
    time: Res<Time>,
) {
    for (entity, mut health) in health_query.iter_mut() {
        if health.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
            break;
        }

        health.regen_timer.tick(time.delta());

        if health.regen_timer.just_finished() {
            health.health += health.regen;
        }

        if health.health > health.max_health {
            health.health = health.max_health;
        }
    }
}
