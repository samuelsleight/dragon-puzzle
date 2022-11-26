use bevy::{app::PluginGroupBuilder, ecs::schedule::StateData, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::util::prelude::*;

pub mod background;
pub mod dragon;
pub mod wall;

pub struct EntityPlugins;

impl<State: StateData> AssetProvider<State> for EntityPlugins {
    fn provide(&self, state: LoadingState<State>) -> LoadingState<State> {
        state.with_asset_provider(dragon::DragonPlugin)
    }
}

impl PluginGroup for EntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(background::BackgroundPlugin)
            .add(wall::WallPlugin)
            .add(dragon::DragonPlugin)
    }
}
