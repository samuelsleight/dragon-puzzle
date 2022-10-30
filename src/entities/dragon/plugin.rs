use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;

use crate::{grid::GridStage, level::WinTimer, util::prelude::*, State};

use super::{
    assets::DragonAssets,
    loadable::DragonBundle,
    systems::{check_win, dragon_movement, finish_movement, rotate_dragons},
};

pub struct DragonPlugin;

impl<State: StateData> AssetProvider<State> for DragonPlugin {
    fn provide(&self, state: LoadingState<State>) -> LoadingState<State> {
        state.with_collection::<DragonAssets>()
    }
}

impl Plugin for DragonPlugin {
    fn build(&self, app: &mut App) {
        app.register_loadable::<DragonBundle>()
            .add_stage_before(
                GridStage,
                "EntityProcessing",
                SystemStage::parallel()
                    .with_system(rotate_dragons)
                    .with_system(finish_movement)
                    .with_system(
                        check_win
                            .run_unless_resource_exists::<WinTimer>()
                            .run_in_state(State::InLevel),
                    ),
            )
            .add_stage_before(
                "EntityProcessing",
                "InputHandling",
                SystemStage::parallel().with_system_set(
                    ConditionSet::new()
                        .run_in_state(State::InLevel)
                        .with_system(dragon_movement)
                        .into(),
                ),
            );
    }
}
