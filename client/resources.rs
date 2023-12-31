use bevy::prelude::*;
use bevy_asset_loader::prelude::*;


#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "explosion.png")]
    pub explosion: Handle<Image>,
}
