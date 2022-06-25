use bevy::prelude::*;

use super::spawn_villager;
use crate::constants::{
    self, VILLAGER_BASE_SPAWN_DURATION, VILLAGER_SPAWN_HOURS, VILLAGER_SPAWN_LOCATIONS,
};
use crate::day_cycle::DayCycleResource;
use rand::Rng;
use std::time::Duration;

pub struct VillagerSpawner {
    pub spawn_timer: Timer,
}

pub fn spawn_villagers_system(
    mut commands: Commands,
    time: Res<Time>,
    day_cycle: Res<DayCycleResource>,
    mut villager_spawner: ResMut<VillagerSpawner>,
    asset_server: Res<AssetServer>,
) {
    if day_cycle.get_hour() > VILLAGER_SPAWN_HOURS.0
        && day_cycle.get_hour() < VILLAGER_SPAWN_HOURS.1
    {
        villager_spawner.spawn_timer.tick(time.delta());

        if villager_spawner.spawn_timer.just_finished() {
            // choose random position in village
            let position: Vec2 =
                VILLAGER_SPAWN_LOCATIONS[rand::thread_rng().gen_range(0..6)].into();

            info!(
                "spawning villager at position {} at time {}",
                position,
                day_cycle.get_hour()
            );

            spawn_villager(position, &mut commands, &asset_server);

            let new_spawn_duration = VILLAGER_BASE_SPAWN_DURATION
                * (-(day_cycle.days_passed as f32 + 1.0) / constants::VILLAGER_SPAWN_FACTOR).exp();

            villager_spawner
                .spawn_timer
                .set_duration(Duration::from_secs_f32(new_spawn_duration));
            villager_spawner.spawn_timer.reset();

            info!("new villager spawn duration set to: {}", new_spawn_duration);
        }
    }
}
