use std::f32::consts::PI;

use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_asset_loader::prelude::{LoadingState, *};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{grid, Action, AssetProvider, Direction, LoadTaskCount, State};

#[derive(Clone, Debug)]
pub struct SpawnDragon {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

#[derive(Component)]
pub struct DragonHead;

pub struct DragonPlugin;

#[derive(AssetCollection)]
struct DragonAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 2, rows = 1))]
    #[asset(path = "dragon.png")]
    atlas: Handle<TextureAtlas>,
}

fn spawn_dragon(
    mut commands: Commands,
    mut events: EventReader<SpawnDragon>,
    mut task_count: ResMut<LoadTaskCount>,
    assets: Res<DragonAssets>,
) {
    for event in events.iter() {
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
            .insert(event.direction)
            .insert(grid::GridPosition {
                x: event.x,
                y: event.y,
            });

        task_count.as_mut().0 -= 1;
    }
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

impl<State: StateData> AssetProvider<State> for DragonPlugin {
    fn provide(&self, state: LoadingState<State>) -> LoadingState<State> {
        state.with_collection::<DragonAssets>()
    }
}

impl Plugin for DragonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnDragon>()
            .add_system(
                spawn_dragon
                    .run_on_event::<SpawnDragon>()
                    .run_in_state(State::LevelLoading),
            )
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
            );
    }
}
