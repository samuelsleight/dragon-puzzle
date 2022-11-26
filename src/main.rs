use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, window::WindowDescriptor,
    DefaultPlugins,
};
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
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::Rgba {
                red: 0.15,
                green: 0.55,
                blue: 0.35,
                alpha: 1.0,
            }),
        },
        ..Default::default()
    });
}

fn main() {
    #[rustfmt::skip]
    App::new()
        // Setup window / application settings
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 800.0,
                        height: 600.0,
                        title: "🐉".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )

        // Add dependent plugins
        .add_plugin(InputManagerPlugin::<Action>::default())

        // Setup the asset loading states
        .add_loopless_state(State::AssetLoading)
        .add_loading_state(
            LoadingState::new(State::AssetLoading)
                .continue_to_state(State::LevelLoading)
                .with_asset_provider(level::LevelPlugin)
                .with_asset_provider(entities::EntityPlugins),
        )

        // Add ordered stages for system management
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

        // Install all game plugins
        .add_plugin(level::LevelPlugin)
        .add_plugin(grid::GridPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugins(entities::EntityPlugins)

        // Setup the camera
        .add_exit_system(State::AssetLoading, spawn_camera)

        // Run the game 🚀
        .run();
}
