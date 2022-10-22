use bevy::{ecs::schedule::StateData, prelude::*, reflect::TypeUuid};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use iyes_loopless::prelude::*;

use crate::{dragon, grid, AssetProvider, Direction, State};

pub struct LevelPlugin;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoadTaskCount(pub usize);

#[derive(serde::Deserialize, Clone, Copy)]
struct DragonConfig {
    position: [i32; 2],
    direction: Direction,
}

#[derive(serde::Deserialize, TypeUuid, Clone)]
#[uuid = "8d84e066-5bad-49f1-85d1-60788779f1d5"]
struct LevelConfig {
    size: [u32; 2],
    dragons: Vec<DragonConfig>,
}

#[derive(AssetCollection)]
struct LevelAssets {
    #[asset(path = "levels", collection(typed))]
    levels: Vec<Handle<LevelConfig>>,
}

fn load_level(
    mut commands: Commands,
    mut dragon_events: EventWriter<dragon::SpawnDragon>,
    config: Res<LevelAssets>,
    assets: Res<Assets<LevelConfig>>,
) {
    let handle = config.levels.first().unwrap();
    let level = assets.get(handle).unwrap();

    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(grid::GridBundle {
        size: grid::GridSize::new(level.size[0], level.size[1]),
        scale: grid::GridScale::new_square(32.0),
    });

    let event_count = level.dragons.len();
    commands.insert_resource(LoadTaskCount(event_count));

    dragon_events.send_batch(level.dragons.iter().map(|config| dragon::SpawnDragon {
        x: config.position[0],
        y: config.position[1],
        direction: config.direction,
    }));
}

fn finish_level_load(mut commands: Commands) {
    commands.remove_resource::<LoadTaskCount>();
    commands.insert_resource(NextState(State::InLevel));
}

impl<State: StateData> AssetProvider<State> for LevelPlugin {
    fn provide(&self, state: LoadingState<State>) -> LoadingState<State> {
        state.with_collection::<LevelAssets>()
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(JsonAssetPlugin::<LevelConfig>::new(&["level"]))
            .add_enter_system(State::LevelLoading, load_level)
            .add_system(
                finish_level_load
                    .run_in_state(State::LevelLoading)
                    .run_if_resource_equals(LoadTaskCount(0)),
            );
    }
}
