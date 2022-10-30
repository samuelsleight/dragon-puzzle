use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    stage::{EntityFinalisationStage, InputHandlingStage},
    util::prelude::*,
    State,
};

use super::{
    loadable::MovementBundle,
    systems::{finish_movement, process_movement},
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_loadable::<MovementBundle>()
            .add_system_set_to_stage(
                InputHandlingStage,
                ConditionSet::new()
                    .run_in_state(State::InLevel)
                    .with_system(process_movement)
                    .into(),
            )
            .add_system_set_to_stage(
                EntityFinalisationStage,
                ConditionSet::new()
                    .run_in_state(State::InLevel)
                    .with_system(finish_movement)
                    .into(),
            );
    }
}
