use bevy::{ecs::schedule::StateData, prelude::*, reflect::TypeUuid};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    action::Action,
    util::{self, prelude::*},
    Direction, State,
};

pub struct LevelPlugin;

#[derive(Clone, Debug)]
pub struct WinTimer(pub Timer);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentLevel(pub usize);

#[derive(serde::Deserialize, Clone, Copy)]
pub struct DragonConfig {
    pub position: [i32; 2],
    pub direction: Direction,
}

#[derive(serde::Deserialize, TypeUuid, Clone)]
#[uuid = "8d84e066-5bad-49f1-85d1-60788779f1d5"]
pub struct LevelConfig {
    pub size: [u32; 2],
    pub dragons: Vec<DragonConfig>,
}

#[cfg(target_family = "wasm")]
#[derive(AssetCollection)]
struct LevelAssets {
    #[asset(
        paths("levels/1.level", "levels/2.level", "levels/3.level"),
        collection(typed)
    )]
    levels: Vec<Handle<LevelConfig>>,
}

#[cfg(not(target_family = "wasm"))]
#[derive(AssetCollection)]
struct LevelAssets {
    #[asset(path = "levels", collection(typed))]
    levels: Vec<Handle<LevelConfig>>,
}

#[derive(Component)]
struct LevelSwitcher;

#[derive(Component)]
pub struct LevelComponent;

#[derive(Bundle)]
struct LevelSwitcherBundle {
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

fn load_level(world: &mut World) {
    world.resource_scope(|world, config: Mut<LevelAssets>| {
        let index = world.resource_scope(|_, mut current: Mut<CurrentLevel>| {
            let index = current.0 % config.levels.len();
            current.0 += 1;
            index
        });

        world.resource_scope(|world, assets: Mut<Assets<LevelConfig>>| {
            let handle = &config.levels[index];
            let level = assets.get(handle).unwrap();

            util::load_loadables(world, level);
        });
    });

    world.insert_resource(NextState(State::InLevel));
}

fn switch_level(mut commands: Commands, query: Query<&ActionState<Action>, With<LevelSwitcher>>) {
    let action = query.single();

    for action in action.get_just_released() {
        if let Action::SwitchLevel = action {
            commands.insert_resource(NextState(State::LevelLoading));
        }
    }
}

fn check_win_timer(mut commands: Commands, time: Res<Time>, mut timer: ResMut<WinTimer>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        commands.insert_resource(NextState(State::LevelLoading));
    }
}

fn unload_level(mut commands: Commands, mut level_query: Query<Entity, With<LevelComponent>>) {
    commands.remove_resource::<WinTimer>();

    for item in level_query.iter_mut() {
        commands.entity(item).despawn();
    }
}

impl<State: StateData> AssetProvider<State> for LevelPlugin {
    fn provide(&self, state: LoadingState<State>) -> LoadingState<State> {
        state.with_collection::<LevelAssets>()
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(JsonAssetPlugin::<LevelConfig>::new(&["level"]))
            .register_loadable::<LevelSwitcherBundle>()
            .add_enter_system(State::LevelLoading, load_level.exclusive_system())
            .add_exit_system(State::InLevel, unload_level)
            .add_system(switch_level.run_in_state(State::InLevel))
            .add_system(
                check_win_timer
                    .run_if_resource_exists::<WinTimer>()
                    .run_in_state(State::InLevel),
            )
            .insert_resource(CurrentLevel(0));
    }
}
