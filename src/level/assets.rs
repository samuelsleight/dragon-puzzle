use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::config::LevelConfig;

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[cfg_attr(
        target_family = "wasm",
        asset(
            paths("levels/1.level", "levels/2.level", "levels/3.level"),
            collection(typed)
        )
    )]
    #[cfg_attr(not(target_family = "wasm"), asset(path = "levels", collection(typed)))]
    pub levels: Vec<Handle<LevelConfig>>,
}
