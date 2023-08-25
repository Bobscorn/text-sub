use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::constants::{MOTHERSHIP_MAX_WIDTH, MOTHERSHIP_MAX_HEIGHT, EMPTY_CHAR};


#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "explosion.png")]
    pub explosion: Handle<Image>,
}

#[derive(Resource)]
pub struct Colors {
    pub normal_text: Color,
    pub ship_text: Color,
    pub menu_background: Color,
    pub node_background: Color,
    pub button_normal: Color,
    pub button_pressed: Color
}

#[derive(Resource)]
pub struct Ship {
    pub pieces: [[char; MOTHERSHIP_MAX_HEIGHT]; MOTHERSHIP_MAX_WIDTH]
}

impl Default for Ship {
    fn default() -> Self {
        Ship{ pieces: [[EMPTY_CHAR; MOTHERSHIP_MAX_HEIGHT]; MOTHERSHIP_MAX_WIDTH] }
    }
}

#[derive(Resource)]
pub struct ShipbuilderShip {
    pub root: Entity,
    pub pieces: [[Option<Entity>; MOTHERSHIP_MAX_HEIGHT]; MOTHERSHIP_MAX_WIDTH]
}

impl Default for ShipbuilderShip {
    fn default() -> Self {
        ShipbuilderShip { root: Entity::PLACEHOLDER, pieces: [[None; MOTHERSHIP_MAX_HEIGHT]; MOTHERSHIP_MAX_WIDTH] }
    }
}
