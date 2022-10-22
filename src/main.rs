use bevy::{
    ecs::schedule::StateData, prelude::*, render::texture::ImageSettings, window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

mod dragon;
mod grid;

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

#[derive(Actionlike, Clone, Copy, Hash, Debug)]
enum Action {
    Forwards,
    TurnLeft,
    TurnRight,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum State {
    AssetLoading,
    LevelLoading,
    InLevel,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoadTaskCount(pub usize);

#[derive(Component, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn process_action(&mut self, action: Action) -> Self {
        match action {
            Action::Forwards => (),
            Action::TurnRight => match *self {
                Direction::Up => *self = Direction::Left,
                Direction::Down => *self = Direction::Right,
                Direction::Left => *self = Direction::Down,
                Direction::Right => *self = Direction::Up,
            },
            Action::TurnLeft => match *self {
                Direction::Up => *self = Direction::Right,
                Direction::Down => *self = Direction::Left,
                Direction::Left => *self = Direction::Up,
                Direction::Right => *self = Direction::Down,
            },
        };

        *self
    }
}

fn load_level(mut commands: Commands, mut dragon_events: EventWriter<dragon::SpawnDragon>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(grid::GridBundle {
        size: grid::GridSize::new_square(10),
        scale: grid::GridScale::new_square(32.0),
    });

    let dragons = [
        dragon::SpawnDragon {
            x: 10,
            y: 7,
            direction: Direction::Left,
        },
        dragon::SpawnDragon {
            x: 0,
            y: 3,
            direction: Direction::Right,
        },
        dragon::SpawnDragon {
            x: 7,
            y: 10,
            direction: Direction::Up,
        },
        dragon::SpawnDragon {
            x: 3,
            y: 0,
            direction: Direction::Down,
        },
    ];

    let event_count = dragons.len();
    commands.insert_resource(LoadTaskCount(event_count));

    dragon_events.send_batch(dragons.into_iter());
}

fn finish_level_load(mut commands: Commands) {
    commands.remove_resource::<LoadTaskCount>();
    commands.insert_resource(NextState(State::InLevel));
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "🐉".into(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.15,
            green: 0.55,
            blue: 0.35,
            alpha: 1.0,
        }))
        .insert_resource(ImageSettings::default_nearest())
        .add_loopless_state(State::AssetLoading)
        .add_loading_state(
            LoadingState::new(State::AssetLoading)
                .continue_to_state(State::LevelLoading)
                .with_asset_provider(dragon::DragonPlugin),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(grid::GridPlugin)
        .add_plugin(dragon::DragonPlugin)
        .add_enter_system(State::LevelLoading, load_level)
        .add_system(
            finish_level_load
                .run_in_state(State::LevelLoading)
                .run_if_resource_equals(LoadTaskCount(0)),
        )
        .run()
}
