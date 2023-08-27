use std::{default, f32::consts::PI};

use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::resources::Submarine;

#[derive(PartialEq)]
pub enum ButtonType {
    PlayButton,
    SubBuilderButton,
    BackToMenuButton,
    SaveButton
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    SubBuilding,
    MainMenu,
    MatchMaking,
    SubSyncing,
    InGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, Reflect)]
pub enum PieceRotation {
    #[default]
    North,
    East,
    South,
    West
}

impl PieceRotation {
    pub fn default() -> PieceRotation {
        PieceRotation::North
    }

    pub fn rotated_cw(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East  => Self::South,
            Self::South => Self::West,
            Self::West  => Self::North
        }
    }

    pub fn rotated_ccw(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West  => Self::South,
            Self::South => Self::East,
            Self::East  => Self::North
        }
    }

    pub fn rotation_radians(&self) -> f32 {
        match self {
            Self::North => 0.0,
            Self::East => PI * 1.5,
            Self::South => PI,
            Self::West => PI * 0.5
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SyncShipsMessageType {
    SyncShip(Submarine),
    SyncShipAck,
    ReadyToStart,
    ReadyToStartAck
}
