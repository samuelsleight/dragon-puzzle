use bevy::prelude::*;

use crate::{
    grid::GridPosition,
    level::{LevelComponent, LevelConfig},
    movement::Movement,
    util::prelude::*,
    Direction,
};

use super::{assets::DragonAssets, components::DragonHead};

#[derive(Bundle)]
pub struct DragonBundle {
    head: DragonHead,
    component: LevelComponent,
    direction: Direction,
    position: GridPosition,
    movement: Movement,

    #[bundle]
    sprite_sheet: SpriteSheetBundle,
}

impl DragonBundle {
    fn new(direction: Direction, position: GridPosition, atlas: Handle<TextureAtlas>) -> Self {
        Self {
            head: DragonHead,
            component: LevelComponent,
            direction,
            position,
            movement: Movement::default(),
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: atlas,
                ..Default::default()
            },
        }
    }
}

impl Loadable<LevelConfig> for DragonBundle {
    fn from_scene(world: &mut World, level: &LevelConfig) {
        world.resource_scope(|world, assets: Mut<DragonAssets>| {
            let atlas = assets.atlas.clone();

            world.spawn_batch(level.dragons.iter().map(move |dragon| {
                DragonBundle::new(
                    dragon.direction,
                    GridPosition::new(dragon.position[0], dragon.position[1]),
                    atlas.clone(),
                )
            }));
        });
    }
}
