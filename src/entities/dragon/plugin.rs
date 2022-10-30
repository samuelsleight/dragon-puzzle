use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;

use crate::{level::WinTimer, stage::EntityProcessingStage, util::prelude::*, State};

use super::{
    assets::DragonAssets,
    loadable::DragonBundle,
    systems::{check_win, rotate_dragons, spawn_body},
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
            .add_system_set_to_stage(
                EntityProcessingStage,
                SystemSet::new()
                    .with_system(rotate_dragons)
                    .with_system(spawn_body.run_in_state(State::InLevel))
                    .with_system(
                        check_win
                            .run_unless_resource_exists::<WinTimer>()
                            .run_in_state(State::InLevel),
                    ),
            );
    }
}
