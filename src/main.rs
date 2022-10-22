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

#[derive(Copy, Clone)]
enum Action {
    Forwards,
    TurnLeft,
    TurnRight,
}

#[derive(Component, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
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

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.atlas.clone(),
            ..Default::default()
        })
        .insert(DragonHead)
        .insert(Direction::Right)
        .insert(Position { x: 0, y: 3 });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.atlas.clone(),
            ..Default::default()
        })
        .insert(DragonHead)
        .insert(Direction::Left)
        .insert(Position { x: 10, y: 7 });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.atlas.clone(),
            ..Default::default()
        })
        .insert(DragonHead)
        .insert(Direction::Up)
        .insert(Position { x: 7, y: 10 });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.atlas.clone(),
            ..Default::default()
        })
        .insert(DragonHead)
        .insert(Direction::Down)
        .insert(Position { x: 3, y: 0 });
}

fn dragon_movement(
    mut commands: Commands,
    assets: Res<DragonAssets>,
    input: Res<Input<KeyCode>>,
    mut dragons: Query<(&mut Direction, &mut Position), With<DragonHead>>,
) {
    let action = if input.just_released(KeyCode::W) || input.just_released(KeyCode::Up) {
        Action::Forwards
    } else if input.just_released(KeyCode::A) || input.just_released(KeyCode::Left) {
        Action::TurnLeft
    } else if input.just_released(KeyCode::D) || input.just_released(KeyCode::Right) {
        Action::TurnRight
    } else {
        return;
    };

    for (mut direction, mut position) in dragons.iter_mut() {
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

fn position_dragons(mut q: Query<(&Position, &Direction, &mut Transform)>) {
    fn convert(grid_coord: i32) -> f32 {
        let max_grid_pixels = 10.0 * 32.0;
        let this_grid_pixels = grid_coord as f32 * 32.0;
        0.0 - (max_grid_pixels / 2.0) + this_grid_pixels
    }

    for (position, direction, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(convert(position.x), convert(position.y), 0.0);

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
        .add_enter_system(State::InLevel, load_level)
        .add_stage_before(
            CoreStage::Update,
            "InputHandling",
            SystemStage::parallel().with_system(dragon_movement.run_in_state(State::InLevel)),
        )
        .add_stage_after(
            "InputHandling",
            "EntityProcessing",
            SystemStage::parallel().with_system(position_dragons.run_in_state(State::InLevel)),
        )
        .run()
}
