use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{action::Action, State};

use super::components::LevelSwitcher;

pub fn switch_level(
    mut commands: Commands,
    query: Query<&ActionState<Action>, With<LevelSwitcher>>,
) {
    let action = query.single();

    for action in action.get_just_released() {
        if let Action::SwitchLevel = action {
            commands.insert_resource(NextState(State::LevelLoading));
        }
    }
}
