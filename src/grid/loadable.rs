use bevy::prelude::{Bundle, World};

use crate::{
    level::{LevelComponent, LevelConfig},
    util::prelude::*,
};

use super::components::{GridScale, GridSize};

#[derive(Bundle, Clone, Copy)]
pub struct GridBundle {
    size: GridSize,
    scale: GridScale,
}

impl Loadable<LevelConfig> for GridBundle {
    fn from_scene(world: &mut World, level: &LevelConfig) {
        world
            .spawn(GridBundle {
                size: GridSize::new(level.size[0], level.size[1]),
                scale: GridScale::new_square(32.0),
            })
            .insert(LevelComponent);
    }
}
