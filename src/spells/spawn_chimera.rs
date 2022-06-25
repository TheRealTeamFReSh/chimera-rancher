use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use rand::Rng;

use crate::{
    assets_manager::AssetsManager, camera::MainCamera, chimeras::spawn_chimera,
    inventory_parts::interaction::InventoryManagement, player::Player,
    sound_manager::SpawnChimeraAudioChannel,
};

use super::SpellKind;

pub fn spawn_chimera_system(
    windows: Res<Windows>,
    camera_query: Query<(&Transform, &Camera), With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    mut commands: Commands,
    assets: Res<AssetsManager>,
    spawn_audio: Res<AudioChannel<SpawnChimeraAudioChannel>>,
    mut inv_man: ResMut<InventoryManagement>,
    mut player_query: Query<(&mut Player, &Transform)>,
) {
    let curr_window = windows.get_primary().unwrap();
    let (camera_gl_transform, camera) = camera_query.iter().next().unwrap();

    let capture_input = mouse_input.just_pressed(MouseButton::Left);

    if let Some((mut player, player_transform)) = player_query.iter_mut().next() {
        if capture_input
            && curr_window.cursor_position().unwrap().y > 75.0
            && matches!(player.active_spell, SpellKind::SpawnChimera)
        {
            let cursor_pos = if let Some(screen_pos) = curr_window.cursor_position() {
                // get the size of the window
                let window_size =
                    Vec2::new(curr_window.width() as f32, curr_window.height() as f32);

                // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

                // matrix for undoing the projection and camera transform
                let ndc_to_world =
                    camera_gl_transform.compute_matrix() * camera.projection_matrix.inverse();

                // use it to convert ndc to world-space coordinates
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

                // reduce it to a 2D value
                let world_pos: Vec2 = world_pos.truncate();

                // use this value
                world_pos
            } else {
                Vec2::ZERO
            };
            // if there are 2 items selected
            if let Some((_, part1)) = inv_man.target_1.selection.clone() {
                if let Some((_, part2)) = inv_man.target_2.selection.clone() {
                    let part_1_idx = player
                        .inventory
                        .chimera_parts
                        .iter()
                        .position(|part| part == &part1)
                        .unwrap();

                    player.inventory.chimera_parts.remove(part_1_idx);

                    let part_2_idx = player
                        .inventory
                        .chimera_parts
                        .iter()
                        .position(|part| part == &part2)
                        .unwrap();

                    player.inventory.chimera_parts.remove(part_2_idx);

                    // reset inv_man
                    inv_man.reset();

                    // play audio
                    spawn_audio.set_playback_rate(rand::thread_rng().gen_range(0.7..1.8));
                    spawn_audio.play(assets.sound_spawn_chimera.clone());

                    spawn_chimera((part1, part2), cursor_pos, &mut commands)
                }
            }
        }
    }
}
