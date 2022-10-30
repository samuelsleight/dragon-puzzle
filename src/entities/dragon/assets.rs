use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct DragonAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 2, rows = 1))]
    #[asset(path = "dragon.png")]
    pub atlas: Handle<TextureAtlas>,
}
