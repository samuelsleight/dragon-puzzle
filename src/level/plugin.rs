use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use iyes_loopless::prelude::*;

use crate::{util::prelude::*, State};

use super::{
    assets::LevelAssets,
    config::LevelConfig,
    resources::{CurrentLevel, WinTimer},
    switcher::LevelSwitcherPlugin,
    systems::{check_win_timer, load_level, unload_level},
};

pub struct LevelPlugin;

impl<State: StateData> AssetProvider<State> for LevelPlugin {
    fn provide(&self, state: LoadingState<State>) -> LoadingState<State> {
        state.with_collection::<LevelAssets>()
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(JsonAssetPlugin::<LevelConfig>::new(&["level"]))
            .add_plugin(LevelSwitcherPlugin)
            .add_enter_system(State::LevelLoading, load_level.exclusive_system())
            .add_exit_system(State::InLevel, unload_level)
            .add_system(
                check_win_timer
                    .run_if_resource_exists::<WinTimer>()
                    .run_in_state(State::InLevel),
            )
            .insert_resource(CurrentLevel(0));
    }
}
