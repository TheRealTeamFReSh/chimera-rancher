use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use rand::Rng;

use crate::{
    assets_manager::AssetsManager,
    chimeras::{ChimeraComponent, ChimeraSprite},
    health::Health,
    player::Player,
    sound_manager::ChimeraHitAudioChannel,
    villagers::{VillagerComponent, VillagerSprite},
};

pub fn villager_attack_system(
    assets: Res<AssetsManager>,
    mut villager_query: Query<(&mut VillagerComponent, &Transform)>,
    mut chimera_query: Query<(&mut Health, &Transform, &Children, &mut ChimeraComponent)>,
    mut player_query: Query<
        (&mut Health, &Transform, &Player, &mut TextureAtlasSprite),
        (
            Without<ChimeraComponent>,
            Without<VillagerComponent>,
            Without<ChimeraSprite>,
        ),
    >,
    mut chimera_sprite_query: Query<&mut Sprite, With<ChimeraSprite>>,
    time: Res<Time>,
    hit_audio: Res<AudioChannel<ChimeraHitAudioChannel>>,
) {
    for (mut villager, villager_transform) in villager_query.iter_mut() {
        villager.attack_timer.tick(time.delta());
        let villager_pos = Vec2::new(
            villager_transform.translation.x,
            villager_transform.translation.y,
        );

        if villager.attack_timer.just_finished() {
            //execute attack on first chimera in range
            if let Some((mut player_health, player_transform, player, mut player_sprite)) =
                player_query.iter_mut().next()
            {
                info!("player_damaged");
                let player_pos = Vec2::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                );
                if villager_pos.distance(player_pos) < villager.stats.range {
                    //hit_audio.set_playback_rate(rand::thread_rng().gen_range(0.3..1.8));
                    //hit_audio.play(assets.sound_hit.clone());
                    player_health.health -= villager.stats.attack;
                    player_sprite.color.set_r(255.0);
                    info!("player_damaged");
                    break;
                }
            }
            for (mut chimera_health, chimera_transform, children, mut chimera) in
                chimera_query.iter_mut()
            {
                let chimera_pos = Vec2::new(
                    chimera_transform.translation.x,
                    chimera_transform.translation.y,
                );

                if villager_pos.distance(chimera_pos) < villager.stats.range {
                    // play sound
                    hit_audio.set_playback_rate(rand::thread_rng().gen_range(0.3..1.8));
                    hit_audio.play(assets.sound_hit.clone());

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

pub fn chimera_attack_system(
    mut chimera_query: Query<(&mut ChimeraComponent, &Transform)>,
    mut villager_query: Query<(&mut Health, &Transform, &Children, &mut VillagerComponent)>,
    mut villager_sprite_query: Query<&mut Sprite, With<VillagerSprite>>,
    time: Res<Time>,
) {
    for (mut chimera, chimera_transform) in chimera_query.iter_mut() {
        chimera.attack_timer.tick(time.delta());
        let chimera_pos = Vec2::new(
            chimera_transform.translation.x,
            chimera_transform.translation.y,
        );

        if chimera.attack_timer.just_finished() {
            for (mut villager_health, villager_transform, children, mut villager) in
                villager_query.iter_mut()
            {
                let villager_pos = Vec2::new(
                    villager_transform.translation.x,
                    villager_transform.translation.y,
                );

                if chimera_pos.distance(villager_pos) < chimera.stats.range {
                    villager_health.health -= chimera.stats.attack;
                    for &child in children.iter() {
                        if let Ok(mut villager_sprite) = villager_sprite_query.get_mut(child) {
                            villager_sprite.color.set_r(255.0);
                            villager.damage_timer.reset();
                        }
                    }
                    break;
                }
            }
        }
    }
}
