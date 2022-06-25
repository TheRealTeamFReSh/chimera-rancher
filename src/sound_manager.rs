use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin};
use rand::Rng;

use crate::states::GameStates;

pub struct SoundChannelsPlugin;

impl Plugin for SoundChannelsPlugin {
    fn build(&self, app: &mut App) {
        // setup
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<SpawnChimeraAudioChannel>()
            .add_audio_channel::<ChimeraHitAudioChannel>()
            .add_audio_channel::<FootstepAudioChannel>()
            .add_audio_channel::<BackgroundAudioChannel>()
            .add_audio_channel::<ChimeraCaptureAudioChannel>()
            .add_audio_channel::<VillagerHitAudioChannel>()
            .add_audio_channel::<ChimeraDeathAudioChannel>()
            .add_audio_channel::<ChimeraAttackAudioChanel>()
            .add_audio_channel::<VillagerAttackAudioChannnel>();

        // on game start
        app.add_system_set(
            SystemSet::on_enter(GameStates::Game).with_system(setup_background_music),
        );
        // on game loop
        app.add_system_set(SystemSet::on_update(GameStates::Game).with_system(background_music));
    }
}

pub struct SpawnChimeraAudioChannel;
pub struct ChimeraHitAudioChannel;
pub struct FootstepAudioChannel;
pub struct BackgroundAudioChannel;
pub struct ChimeraCaptureAudioChannel;
pub struct VillagerHitAudioChannel;
pub struct ChimeraDeathAudioChannel;
pub struct ChimeraAttackAudioChanel;
pub struct VillagerAttackAudioChannnel;

const BACKGROUND_MUSICS: &'static [&str] = &[
    "ambient-piano-ampamp-strings-10711.ogg",
    "inspiring-motivational-mood-14107.ogg",
    "price-of-freedom-33106.ogg",
];

fn setup_background_music(
    asset_server: Res<AssetServer>,
    background_audio: Res<AudioChannel<BackgroundAudioChannel>>,
) {
    let random_music = BACKGROUND_MUSICS[rand::thread_rng().gen_range(0..BACKGROUND_MUSICS.len())];
    let mut music_path = String::from("sounds/background/");
    music_path.push_str(random_music);

    background_audio.set_volume(0.5);
    background_audio.play_looped(asset_server.load(&music_path));
}

// TODO: play other musics once the first is done
fn background_music() {}
