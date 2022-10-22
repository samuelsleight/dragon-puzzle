use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::texture::ImageSettings,
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

mod grid;

#[derive(Actionlike, Clone, Copy, Hash, Debug)]
enum Action {
    Forwards,
    TurnLeft,
    TurnRight,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum State {
    AssetLoading,
    InLevel,
}

#[derive(AssetCollection)]
struct DragonAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 2, rows = 1))]
    #[asset(path = "dragon.png")]
    atlas: Handle<TextureAtlas>,
}

#[derive(Component, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct DragonHead;

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

fn load_level(mut commands: Commands, assets: Res<DragonAssets>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(grid::GridBundle {
        size: grid::GridSize::new_square(10),
        scale: grid::GridScale::new_square(32.0),
    });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.atlas.clone(),
            ..Default::default()
        })
        .insert_bundle(InputManagerBundle::<Action> {
            input_map: InputMap::new([
                (KeyCode::W, Action::Forwards),
                (KeyCode::Up, Action::Forwards),
                (KeyCode::A, Action::TurnLeft),
                (KeyCode::D, Action::TurnRight),
                (KeyCode::Left, Action::TurnLeft),
                (KeyCode::Right, Action::TurnRight),
            ]),
            ..Default::default()
        })
        .insert(DragonHead)
        .insert(Direction::Right)
        .insert(grid::GridPosition { x: 0, y: 3 });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.atlas.clone(),
            ..Default::default()
        })
        .insert_bundle(InputManagerBundle::<Action> {
            input_map: InputMap::new([
                (KeyCode::W, Action::Forwards),
                (KeyCode::Up, Action::Forwards),
                (KeyCode::A, Action::TurnLeft),
                (KeyCode::D, Action::TurnRight),
                (KeyCode::Left, Action::TurnLeft),
                (KeyCode::Right, Action::TurnRight),
            ]),
            ..Default::default()
        })
        .insert(DragonHead)
        .insert(Direction::Left)
        .insert(grid::GridPosition { x: 10, y: 7 });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.atlas.clone(),
            ..Default::default()
        })
        .insert_bundle(InputManagerBundle::<Action> {
            input_map: InputMap::new([
                (KeyCode::W, Action::Forwards),
                (KeyCode::Up, Action::Forwards),
                (KeyCode::A, Action::TurnLeft),
                (KeyCode::D, Action::TurnRight),
                (KeyCode::Left, Action::TurnLeft),
                (KeyCode::Right, Action::TurnRight),
            ]),
            ..Default::default()
        })
        .insert(DragonHead)
        .insert(Direction::Up)
        .insert(grid::GridPosition { x: 7, y: 10 });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.atlas.clone(),
            ..Default::default()
        })
        .insert_bundle(InputManagerBundle::<Action> {
            input_map: InputMap::new([
                (KeyCode::W, Action::Forwards),
                (KeyCode::Up, Action::Forwards),
                (KeyCode::A, Action::TurnLeft),
                (KeyCode::D, Action::TurnRight),
                (KeyCode::Left, Action::TurnLeft),
                (KeyCode::Right, Action::TurnRight),
            ]),
            ..Default::default()
        })
        .insert(DragonHead)
        .insert(Direction::Down)
        .insert(grid::GridPosition { x: 3, y: 0 });
}

fn dragon_movement(
    mut commands: Commands,
    assets: Res<DragonAssets>,
    mut dragons: Query<
        (
            &ActionState<Action>,
            &mut Direction,
            &mut grid::GridPosition,
        ),
        With<DragonHead>,
    >,
) {
    for (action, mut direction, mut position) in dragons.iter_mut() {
        let action = if action.just_released(Action::Forwards) {
            Action::Forwards
        } else if action.just_released(Action::TurnLeft) {
            Action::TurnLeft
        } else if action.just_released(Action::TurnRight) {
            Action::TurnRight
        } else {
            continue;
        };

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..Default::default()
                },
                texture_atlas: assets.atlas.clone(),
                ..Default::default()
            })
            .insert(*direction)
            .insert(*position);

        let (dx, dy) = match direction.process_action(action) {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        position.x += dx;
        position.y += dy;
    }
}

fn rotate_dragons(mut q: Query<(&Direction, &mut Transform)>) {
    for (direction, mut transform) in q.iter_mut() {
        transform.rotation = Quat::from_rotation_z(
            (PI / 180.0)
                * match direction {
                    Direction::Up => 90.0,
                    Direction::Down => 270.0,
                    Direction::Left => 0.0,
                    Direction::Right => 180.0,
                },
        );
    }
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
                .continue_to_state(State::InLevel)
                .with_collection::<DragonAssets>(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_plugin(grid::GridPlugin)
        .add_enter_system(State::InLevel, load_level)
        .add_stage_before(
            grid::GridStage,
            "EntityProcessing",
            SystemStage::parallel().with_system(rotate_dragons.run_in_state(State::InLevel)),
        )
        .add_stage_before(
            "EntityProcessing",
            "InputHandling",
            SystemStage::parallel().with_system_set(
                ConditionSet::new()
                    .run_in_state(State::InLevel)
                    .with_system(dragon_movement)
                    .into(),
            ),
        )
        .run()
}
