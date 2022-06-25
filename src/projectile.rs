use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::states::GameStates;

#[derive(Debug, Component)]
pub struct Projectile {
    pub direction_x: f32,
    pub direction_y: f32,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        // on update
        app.add_system_set(SystemSet::on_update(GameStates::Game).with_system(move_projectile));
    }
}

fn move_projectile(
    time: Res<Time>,
    mut query: Query<(&Projectile, &mut Velocity, &mut TextureAtlasSprite)>,
) {
    for (projectile, mut vel, _) in query.iter_mut() {
        let dt = time.delta_seconds();
        vel.linvel.x += projectile.direction_x * dt;
        vel.linvel.y += projectile.direction_y * dt;
    }
}
