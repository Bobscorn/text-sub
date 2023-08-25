use bevy::prelude::*;

#[derive(PartialEq)]
pub enum ButtonType {
    PlayButton,
    SubBuilderButton,
    BackToMenuButton
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    SubBuilding,
    MainMenu,
    MatchMaking,
    InGame,
}
