use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::config::LevelConfig;

#[cfg(target_family = "wasm")]
#[derive(AssetCollection)]
pub struct LevelAssets {
    #[asset(
        paths("levels/1.level", "levels/2.level", "levels/3.level"),
        collection(typed)
    )]
    pub levels: Vec<Handle<LevelConfig>>,
}

#[cfg(not(target_family = "wasm"))]
#[derive(AssetCollection)]
pub struct LevelAssets {
    #[asset(path = "levels", collection(typed))]
    pub levels: Vec<Handle<LevelConfig>>,
}
