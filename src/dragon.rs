use std::f32::consts::PI;

use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_asset_loader::prelude::{LoadingState, *};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{action::Action, grid, level, AssetProvider, Direction, State};

#[derive(Clone, Debug)]
pub struct SpawnDragon {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

#[derive(Component)]
pub struct DragonHead;

#[derive(Component)]
pub struct DragonComponent;

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
    mut task_count: ResMut<level::LoadTaskCount>,
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
                    (KeyCode::W, Action::MovementForwards),
                    (KeyCode::Up, Action::MovementForwards),
                    (KeyCode::A, Action::MovementTurnLeft),
                    (KeyCode::D, Action::MovementTurnRight),
                    (KeyCode::Left, Action::MovementTurnLeft),
                    (KeyCode::Right, Action::MovementTurnRight),
                ]),
                ..Default::default()
            })
            .insert(DragonComponent)
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
    grid_query: Query<&grid::GridSize>,
    mut dragons: Query<
        (
            &ActionState<Action>,
            &mut Direction,
            &mut grid::GridPosition,
        ),
        With<DragonHead>,
    >,
) {
    let movement_max = grid_query.get_single().ok();

    for (action, mut direction, mut position) in dragons.iter_mut() {
        for action in action.get_just_released() {
            let action = match action.movement() {
                Some(action) => action,
                _ => continue,
            };

            let proposed_direction = direction.process_action(action);
            let (dx, dy) = proposed_direction.delta();

            let proposed_x = position.x + dx;
            let proposed_y = position.y + dy;

            if let Some(max) = movement_max {
                if proposed_x < 0
                    || proposed_x >= max.width as i32
                    || proposed_y < 0
                    || proposed_y >= max.height as i32
                {
                    continue;
                }
            }

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: 1,
                        ..Default::default()
                    },
                    texture_atlas: assets.atlas.clone(),
                    ..Default::default()
                })
                .insert(DragonComponent)
                .insert(*direction)
                .insert(*position);

            *direction = proposed_direction;
            position.x = proposed_x;
            position.y = proposed_y;
        }
    }
}

fn rotate_dragons(mut q: Query<(&Direction, &mut Transform)>) {
    for (direction, mut transform) in q.iter_mut() {
        transform.rotation = Quat::from_rotation_z(
            (PI / 180.0)
                * match direction {
                    Direction::Up => 270.0,
                    Direction::Down => 90.0,
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
