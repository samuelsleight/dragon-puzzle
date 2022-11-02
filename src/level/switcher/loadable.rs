use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    action::Action,
    level::{components::LevelComponent, config::LevelConfig},
    util::prelude::*,
};

use super::components::LevelSwitcher;

#[derive(Bundle)]
pub struct LevelSwitcherBundle {
    switcher: LevelSwitcher,
    component: LevelComponent,

    #[bundle]
    input_manager: InputManagerBundle<Action>,
}

impl LevelSwitcherBundle {
    fn new() -> Self {
        LevelSwitcherBundle {
            switcher: LevelSwitcher,
            component: LevelComponent,
            input_manager: InputManagerBundle::<Action> {
                input_map: InputMap::new([(KeyCode::Space, Action::SwitchLevel)]),
                ..Default::default()
            },
        }
    }
}

impl Loadable<LevelConfig> for LevelSwitcherBundle {
    fn from_scene(world: &mut World, _: &LevelConfig) {
        world.spawn_batch([LevelSwitcherBundle::new()]);
    }
}
