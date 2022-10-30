use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    action::Action,
    level::{LevelComponent, LevelConfig},
    util::prelude::*,
};

use super::components::MovementManager;

#[derive(Bundle)]
pub struct MovementBundle {
    manager: MovementManager,
    component: LevelComponent,

    #[bundle]
    input_manager: InputManagerBundle<Action>,
}

impl MovementBundle {
    pub fn new() -> Self {
        Self {
            manager: MovementManager,
            component: LevelComponent,

            input_manager: InputManagerBundle::<Action> {
                input_map: InputMap::new([
                    (KeyCode::W, Action::MovementForwards),
                    (KeyCode::Up, Action::MovementForwards),
                    (KeyCode::A, Action::MovementTurnLeft),
                    (KeyCode::D, Action::MovementTurnRight),
                    (KeyCode::Left, Action::MovementTurnLeft),
                    (KeyCode::Right, Action::MovementTurnRight),
                ]),
                ..Default::default()
            },
        }
    }
}

impl Loadable<LevelConfig> for MovementBundle {
    fn from_scene(world: &mut World, _: &LevelConfig) {
        world.spawn_batch([MovementBundle::new()]);
    }
}
