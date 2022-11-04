use bevy::prelude::*;

use crate::{
    grid::GridPosition,
    level::{LevelComponent, LevelConfig},
    movement::Blocker,
    util::prelude::*,
};

#[derive(Bundle)]
pub struct WallBundle {
    component: LevelComponent,
    position: GridPosition,
    blocker: Blocker,

    #[bundle]
    sprite: SpriteBundle,
}

impl WallBundle {
    pub fn new(position: GridPosition) -> Self {
        WallBundle {
            component: LevelComponent,
            position,
            blocker: Blocker,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.7, 0.5, 0.5, 0.8),
                    ..Default::default()
                },
                transform: Transform {
                    scale: Vec3::new(31.0, 31.0, 31.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

impl Loadable<LevelConfig> for WallBundle {
    fn from_scene(world: &mut World, scene: &LevelConfig) {
        world.spawn_batch(scene.walls.iter().flat_map(|wall_range| {
            (wall_range.from[0]..=wall_range.to[0]).flat_map(|x| {
                (wall_range.from[1]..=wall_range.to[1])
                    .map(move |y| WallBundle::new(GridPosition::new(x, y)))
            })
        }));
    }
}
