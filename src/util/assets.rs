use bevy::ecs::schedule::StateData;
use bevy_asset_loader::prelude::LoadingState;

pub trait AssetProvider<State: StateData> {
    fn provide(&self, state: LoadingState<State>) -> LoadingState<State>;
}

pub trait LoadingStateExt<State: StateData> {
    fn with_asset_provider<Provider: AssetProvider<State>>(self, provider: Provider) -> Self;
}

impl<State: StateData> LoadingStateExt<State> for LoadingState<State> {
    fn with_asset_provider<Provider: AssetProvider<State>>(self, provider: Provider) -> Self {
        provider.provide(self)
    }
}
