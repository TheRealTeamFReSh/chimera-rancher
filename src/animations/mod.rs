use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::chimeras::ChimeraComponent;

use crate::animals::AnimalComponent;
use crate::constants;
use crate::states::GameStates;
use crate::villagers::VillagerComponent;

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameStates::Game).with_system(bob_animation));
    }
}

#[derive(Component)]
pub struct BobbingAnim {
    pub anim: f32,
}

pub fn bob_animation(
    time: Res<Time>,
    q_velocity: Query<
        &Velocity,
        Or<(
            With<AnimalComponent>,
            With<ChimeraComponent>,
            With<VillagerComponent>,
        )>,
    >,
    mut q_bobbing: Query<(&Parent, &mut Transform, &mut BobbingAnim), With<Sprite>>,
) {
    for (parent, mut transform, mut animation) in q_bobbing.iter_mut() {
        // fetch velocity from parent
        let velocity = q_velocity.get(parent.0).unwrap();

        // compute speed from velocity and offsets
        let speed = velocity.linvel.length();
        let offset_y = (animation.anim.sin() * constants::ANIMATION_OFFSET_FACTOR).abs();
        let offset_x = (animation.anim.sin() * constants::ANIMATION_OFFSET_FACTOR).abs();

        // update the animation circle (sin(2*pi) = sin(0))
        animation.anim +=
            (speed * time.delta_seconds() * constants::ANIMATION_SPEED_FACTOR) % (16.0 * 2.0 * PI);

        // apply the transform to the component
        transform.translation.y = offset_y;
        transform.scale.x = transform.scale.x / transform.scale.x.abs() * (1.0 - offset_x * 0.02);
    }
}
