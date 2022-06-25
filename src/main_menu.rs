use bevy::prelude::*;

use crate::states::GameStates;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // on enter
        app.add_system_set(SystemSet::on_enter(GameStates::MainMenu).with_system(build_ui));

        // on update
        app.add_system_set(SystemSet::on_update(GameStates::MainMenu).with_system(start_game));

        // on exit
        app.add_system_set(SystemSet::on_exit(GameStates::MainMenu).with_system(destroy_ui));
    }
}

#[derive(Component)]
pub struct MainMenuUI;

fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ImageBundle {
            image: asset_server.load("main_menu.png").into(), // not using assetsmanager as we don't load everything on the main menu
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            ..default()
        })
        .insert(MainMenuUI);
}

fn start_game(mut keyboard: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<GameStates>>) {
    if keyboard.just_pressed(KeyCode::Return) {
        game_state.set(GameStates::Game).unwrap();
        keyboard.reset(KeyCode::Return);
    }
}

fn destroy_ui(mut commands: Commands, q_ui: Query<Entity, With<MainMenuUI>>) {
    for entity in q_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
