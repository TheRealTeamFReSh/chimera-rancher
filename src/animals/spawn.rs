use bevy::prelude::*;

use crate::animals::{spawn_animal, AnimalAttributesResource, AnimalKind};
use crate::constants::{self, ANIMAL_BASE_SPAWN_DURATION, ANIMAL_SPAWN_HOURS};
use crate::day_cycle::DayCycleResource;
use rand::Rng;
use std::time::Duration;

pub struct AnimalSpawner {
    pub spawn_timer: Timer,
}

pub fn spawn_animals_system(
    mut commands: Commands,
    time: Res<Time>,
    day_cycle: Res<DayCycleResource>,
    mut animal_spawner: ResMut<AnimalSpawner>,
    animal_attr_res: Res<AnimalAttributesResource>,
    asset_server: Res<AssetServer>,
) {
    if day_cycle.get_hour() > ANIMAL_SPAWN_HOURS.0 || day_cycle.get_hour() < ANIMAL_SPAWN_HOURS.1 {
        animal_spawner.spawn_timer.tick(time.delta());

        if animal_spawner.spawn_timer.just_finished() {
            let random_animal: AnimalKind = rand::random();

            // choose random position in village
            let rand_x_pos: f32 = rand::thread_rng()
                .gen_range(constants::ANIMAL_SPAWN_MIN_X..=constants::ANIMAL_SPAWN_MAX_X);
            let rand_y_pos: f32 = rand::thread_rng()
                .gen_range(constants::ANIMAL_SPAWN_MIN_Y..=constants::ANIMAL_SPAWN_MAX_Y);
            info!(
                "spawning {:?} at position ({},{}) at time {}",
                random_animal,
                rand_x_pos,
                rand_y_pos,
                day_cycle.get_hour()
            );
            spawn_animal(
                &random_animal,
                Vec2::new(rand_x_pos, rand_y_pos),
                &animal_attr_res,
                &mut commands,
                &asset_server,
            );

            let new_spawn_duration = ANIMAL_BASE_SPAWN_DURATION
                * (-(day_cycle.days_passed as f32 + 1.0) / constants::ANIMAL_SPAWN_FACTOR).exp();

            animal_spawner
                .spawn_timer
                .set_duration(Duration::from_secs_f32(new_spawn_duration));
            animal_spawner.spawn_timer.reset();

            info!("new animal spawn duration set to: {}", new_spawn_duration);
        }
    }
}
