use bevy::{prelude::*, render::texture::ImageSettings, window::WindowDescriptor, DefaultPlugins};
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    action::Action,
    stage::{EntityFinalisationStage, EntityProcessingStage, InputHandlingStage},
    util::prelude::*,
};

mod action;
mod direction;
mod entities;
mod grid;
mod level;
mod movement;
mod stage;
mod util;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum State {
    AssetLoading,
    LevelLoading,
    InLevel,
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
                .with_asset_provider(entities::EntityPlugins),
        )
        .add_plugins(DefaultPlugins)
        .add_stage_before(
            CoreStage::PostUpdate,
            EntityFinalisationStage,
            SystemStage::parallel(),
        )
        .add_stage_before(
            EntityFinalisationStage,
            EntityProcessingStage,
            SystemStage::parallel(),
        )
        .add_stage_before(
            EntityProcessingStage,
            InputHandlingStage,
            SystemStage::parallel(),
        )
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(level::LevelPlugin)
        .add_plugin(grid::GridPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugins(entities::EntityPlugins)
        .add_exit_system(State::AssetLoading, spawn_camera)
        .run();
}
