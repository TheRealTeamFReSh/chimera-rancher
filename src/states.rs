use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub enum GameStates {
    MainMenu,
    AssetsLoading,
    PauseMenu,
    Game,
    GameOver,
}
