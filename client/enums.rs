use bevy::prelude::*;

#[derive(PartialEq)]
pub enum ButtonType {
    PlayButton,
    subBuilderButton,
    BackToMenuButton
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    subBuilding,
    MainMenu,
    MatchMaking,
    InGame,
}
