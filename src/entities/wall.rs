use bevy::prelude::*;

use crate::{
    grid::GridPosition,
    level::{LevelComponent, LevelConfig},
    util::prelude::*,
};

pub struct WallPlugin;

#[derive(Component)]
pub struct Blocker;

#[derive(Bundle)]
struct WallBundle {
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
        world.spawn_batch([
            WallBundle::new(GridPosition { x: 1, y: 1 }),
            WallBundle::new(GridPosition {
                x: scene.size[0] as i32 - 2,
                y: 1,
            }),
        ]);
    }
}

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_loadable::<WallBundle>();
    }
}
