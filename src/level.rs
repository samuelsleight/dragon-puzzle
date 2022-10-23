use bevy::{ecs::schedule::StateData, prelude::*, reflect::TypeUuid};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{action::Action, dragon, grid, AssetProvider, Direction, State};

pub struct LevelPlugin;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoadTaskCount(pub usize);

#[derive(Clone, Debug)]
pub struct WinTimer(pub Timer);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentLevel(pub usize);

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

#[derive(Component)]
struct LevelSwitcher;

#[derive(Component)]
struct LevelComponent;

fn load_level(
    mut commands: Commands,
    mut dragon_events: EventWriter<dragon::SpawnDragon>,
    mut current_level: ResMut<CurrentLevel>,
    config: Res<LevelAssets>,
    assets: Res<Assets<LevelConfig>>,
) {
    let index = current_level.0 % config.levels.len();
    let handle = &config.levels[index];
    let level = assets.get(handle).unwrap();
    current_level.0 += 1;

    commands
        .spawn_bundle(InputManagerBundle::<Action> {
            input_map: InputMap::new([(KeyCode::Space, Action::SwitchLevel)]),
            ..Default::default()
        })
        .insert(LevelComponent)
        .insert(LevelSwitcher);

    commands
        .spawn_bundle(grid::GridBundle {
            size: grid::GridSize::new(level.size[0], level.size[1]),
            scale: grid::GridScale::new_square(32.0),
        })
        .insert(LevelComponent);

    for x in 0..level.size[0] {
        for y in 0..level.size[1] {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.2, 0.2, 0.2, 0.6),
                        ..Default::default()
                    },
                    transform: Transform {
                        scale: Vec3::new(30.0, 30.0, 30.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(LevelComponent)
                .insert(grid::GridPosition {
                    x: x as i32,
                    y: y as i32,
                });
        }
    }

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

fn unload_level(
    mut commands: Commands,
    mut level_query: Query<Entity, With<dragon::DragonComponent>>,
    mut dragon_query: Query<Entity, With<LevelComponent>>,
) {
    commands.remove_resource::<WinTimer>();

    for dragon in dragon_query.iter_mut() {
        commands.entity(dragon).despawn();
    }

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
            .add_enter_system(State::LevelLoading, load_level)
            .add_exit_system(State::InLevel, unload_level)
            .add_system(
                finish_level_load
                    .run_in_state(State::LevelLoading)
                    .run_if_resource_equals(LoadTaskCount(0)),
            )
            .add_system(switch_level.run_in_state(State::InLevel))
            .add_system(
                check_win_timer
                    .run_if_resource_exists::<WinTimer>()
                    .run_in_state(State::InLevel),
            )
            .insert_resource(CurrentLevel(0));
    }
}