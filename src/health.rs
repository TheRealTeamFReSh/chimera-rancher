use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max_health: f32,
    pub health: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self {
            max_health: health,
            health,
        }
    }
}
