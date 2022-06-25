use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    chimeras::{ChimeraComponent, ChimeraSprite},
    health::Health,
    player::Player,
    villagers::VillagerComponent,
};

pub fn villager_attack_system(
    mut villager_query: Query<(&mut VillagerComponent, &Transform)>,
    mut player_query: Query<&mut Health, (With<Player>, Without<ChimeraComponent>)>,
    mut chimera_query: Query<(&mut Health, &Transform, &Children, &mut ChimeraComponent)>,
    mut chimera_sprite_query: Query<&mut Sprite, With<ChimeraSprite>>,
    time: Res<Time>,
) {
    for (mut villager, villager_transform) in villager_query.iter_mut() {
        villager.attack_timer.tick(time.delta());
        let villager_pos = Vec2::new(
            villager_transform.translation.x,
            villager_transform.translation.y,
        );

        if villager.attack_timer.just_finished() {
            //execute attack on first chimera in range
            for (mut chimera_health, chimera_transform, children, mut chimera) in
                chimera_query.iter_mut()
            {
                let chimera_pos = Vec2::new(
                    chimera_transform.translation.x,
                    chimera_transform.translation.y,
                );

                if villager_pos.distance(chimera_pos) < villager.stats.range {
                    chimera_health.health -= villager.stats.attack;
                    for &child in children.iter() {
                        if let Ok(mut chimera_sprite) = chimera_sprite_query.get_mut(child) {
                            chimera_sprite.color.set_r(255.0);
                            chimera.damage_timer.reset();
                        }
                    }
                    break;
                }
            }
        }
    }
}
