use action::{Action, MovementAction};
use bevy::{
    ecs::schedule::StateData, prelude::*, render::texture::ImageSettings, window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

mod action;
mod dragon;
mod grid;
mod level;

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum State {
    AssetLoading,
    LevelLoading,
    InLevel,
}

#[derive(serde::Deserialize, Component, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn process_action(&mut self, action: MovementAction) -> Self {
        match action {
            MovementAction::Forwards => (),
            MovementAction::TurnLeft => match *self {
                Direction::Up => *self = Direction::Left,
                Direction::Down => *self = Direction::Right,
                Direction::Left => *self = Direction::Down,
                Direction::Right => *self = Direction::Up,
            },
            MovementAction::TurnRight => match *self {
                Direction::Up => *self = Direction::Right,
                Direction::Down => *self = Direction::Left,
                Direction::Left => *self = Direction::Up,
                Direction::Right => *self = Direction::Down,
            },
        };

        *self
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
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
                .with_asset_provider(level::LevelPlugin)
                .with_asset_provider(dragon::DragonPlugin),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(grid::GridPlugin)
        .add_plugin(level::LevelPlugin)
        .add_plugin(dragon::DragonPlugin)
        .add_exit_system(State::AssetLoading, spawn_camera)
        .run();
}
