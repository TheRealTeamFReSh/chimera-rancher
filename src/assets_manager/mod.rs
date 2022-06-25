use bevy::prelude::*;
use bevy_asset_loader::*;

use crate::states::GameStates;

pub struct AssetsManagerPlugin;

impl Plugin for AssetsManagerPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameStates::AssetsLoading)
            .continue_to_state(GameStates::MainMenu)
            .with_collection::<AssetsManager>()
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct AssetsManager {
    // fonts
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font_bold: Handle<Font>,
    #[asset(path = "fonts/FiraSans-Regular.ttf")]
    pub font_regular: Handle<Font>,
    // sounds
    #[asset(path = "sounds/footstep.ogg")]
    pub sound_footstep: Handle<bevy_kira_audio::AudioSource>,
    #[asset(path = "sounds/hit.ogg")]
    pub sound_hit: Handle<bevy_kira_audio::AudioSource>,
    #[asset(path = "sounds/spawn_chimera.ogg")]
    pub sound_spawn_chimera: Handle<bevy_kira_audio::AudioSource>,
    #[asset(path = "sounds/button.ogg")]
    pub sound_button: Handle<bevy_kira_audio::AudioSource>,
    // images
    #[asset(path = "lighting.png")]
    pub texture_lightning: Handle<Image>,
    #[asset(path = "hotbar.png")]
    pub texture_hotbar: Handle<Image>,
    #[asset(path = "mage.png")]
    pub texture_mage: Handle<Image>,
    #[asset(path = "ui_background.png")]
    pub texture_ui_background: Handle<Image>,
    #[asset(path = "target.png")]
    pub texture_target: Handle<Image>,
    // chimera parts
    #[asset(path = "pig.png")]
    pub texture_pig: Handle<Image>,
    #[asset(path = "pighead.png")]
    pub texture_pig_head: Handle<Image>,
    #[asset(path = "pigtail.png")]
    pub texture_pig_tail: Handle<Image>,
    #[asset(path = "cow.png")]
    pub texture_cow: Handle<Image>,
    #[asset(path = "cowhead.png")]
    pub texture_cow_head: Handle<Image>,
    #[asset(path = "cowtail.png")]
    pub texture_cow_tail: Handle<Image>,
    #[asset(path = "dog.png")]
    pub texture_dog: Handle<Image>,
    #[asset(path = "doghead.png")]
    pub texture_dog_head: Handle<Image>,
    #[asset(path = "dogtail.png")]
    pub texture_dog_tail: Handle<Image>,
    #[asset(path = "chicken.png")]
    pub texture_chicken: Handle<Image>,
    #[asset(path = "chickenhead.png")]
    pub texture_chicken_head: Handle<Image>,
    #[asset(path = "chickentail.png")]
    pub texture_chicken_tail: Handle<Image>,
    #[asset(path = "horse.png")]
    pub texture_horse: Handle<Image>,
    #[asset(path = "horsehead.png")]
    pub texture_horse_head: Handle<Image>,
    #[asset(path = "horsetail.png")]
    pub texture_horse_tail: Handle<Image>,
}
