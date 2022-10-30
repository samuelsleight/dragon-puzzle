use bevy::prelude::*;

use crate::{
    grid::GridPosition,
    level::{LevelComponent, LevelConfig},
    util::prelude::*,
};

#[derive(Bundle)]
pub struct TileBundle {
    component: LevelComponent,
    position: GridPosition,

    #[bundle]
    sprite: SpriteBundle,
}

impl TileBundle {
    pub fn new(position: GridPosition) -> Self {
        TileBundle {
            component: LevelComponent,
            position,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.2, 0.2, 0.2, 0.6),
                    ..Default::default()
                },
                transform: Transform {
                    scale: Vec3::new(30.0, 30.0, 30.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

impl Loadable<LevelConfig> for TileBundle {
    fn from_scene(world: &mut World, scene: &LevelConfig) {
        world.spawn_batch((0..scene.size[0]).flat_map(|x| {
            (0..scene.size[1]).map(move |y| {
                TileBundle::new(GridPosition {
                    x: x as i32,
                    y: y as i32,
                })
            })
        }));
    }
}
