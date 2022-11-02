use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{util::prelude::*, State};

use super::{loadable::LevelSwitcherBundle, systems::switch_level};

pub struct LevelSwitcherPlugin;

impl Plugin for LevelSwitcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_loadable::<LevelSwitcherBundle>()
            .add_system(switch_level.run_in_state(State::InLevel));
    }
}
