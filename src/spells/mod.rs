use bevy::prelude::*;

use crate::GameStates;

mod fire_projectile;
mod spawn_chimera;
pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStates::Game)
                .with_system(spawn_chimera::spawn_chimera_system)
                .with_system(fire_projectile::fire_projectile_system),
        );
    }
}

#[derive(Debug)]
pub enum SpellKind {
    SpawnChimera,
    FireProjectile,
}
