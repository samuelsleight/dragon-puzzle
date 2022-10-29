use std::f32::consts::PI;

use bevy::{ecs::schedule::StateData, prelude::*};
use bevy_asset_loader::prelude::{LoadingState, *};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    action::Action,
    grid,
    level::{self, LevelConfig},
    util::prelude::*,
    Direction, State,
};

#[derive(Component)]
pub struct DragonHead;

pub struct DragonPlugin;

#[derive(AssetCollection)]
struct DragonAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 2, rows = 1))]
    #[asset(path = "dragon.png")]
    atlas: Handle<TextureAtlas>,
}

#[derive(Bundle)]
pub struct DragonBundle {
    head: DragonHead,
    component: level::LevelComponent,
    direction: Direction,
    position: grid::GridPosition,

    #[bundle]
    sprite_sheet: SpriteSheetBundle,

    #[bundle]
    input_manager: InputManagerBundle<Action>,
}

impl DragonBundle {
    fn new(
        direction: Direction,
        position: grid::GridPosition,
        atlas: Handle<TextureAtlas>,
    ) -> Self {
        Self {
            head: DragonHead,
            component: level::LevelComponent,
            direction,
            position,
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: atlas,
                ..Default::default()
            },
            input_manager: InputManagerBundle::<Action> {
                input_map: InputMap::new([
                    (KeyCode::W, Action::MovementForwards),
                    (KeyCode::Up, Action::MovementForwards),
                    (KeyCode::A, Action::MovementTurnLeft),
                    (KeyCode::D, Action::MovementTurnRight),
                    (KeyCode::Left, Action::MovementTurnLeft),
                    (KeyCode::Right, Action::MovementTurnRight),
                ]),
                ..Default::default()
            },
        }
    }
}

impl Loadable<LevelConfig> for DragonBundle {
    fn from_scene(world: &mut World, level: &LevelConfig) {
        world.resource_scope(|world, assets: Mut<DragonAssets>| {
            let atlas = assets.atlas.clone();

            world.spawn_batch(level.dragons.iter().map(move |dragon| {
                DragonBundle::new(
                    dragon.direction,
                    grid::GridPosition {
                        x: dragon.position[0],
                        y: dragon.position[1],
                    },
                    atlas.clone(),
                )
            }));
        });
    }
}

fn dragon_movement(
    mut commands: Commands,
    assets: Res<DragonAssets>,
    grid_query: Query<&grid::GridSize>,
    mut set: ParamSet<(
        Query<
            (
                &ActionState<Action>,
                &mut Direction,
                &mut grid::GridPosition,
            ),
            With<DragonHead>,
        >,
        Query<&grid::GridPosition, With<level::Blocker>>,
    )>,
) {
    let movement_max = grid_query.get_single().ok();
    let blockers = set.p1().iter().cloned().collect::<Vec<_>>();

    for (action, mut direction, mut position) in set.p0().iter_mut() {
        for action in action.get_just_released() {
            let action = match action.movement() {
                Some(action) => action,
                _ => continue,
            };

            let proposed_direction = direction.process_action(action);
            let proposed_position = position.apply_direction(proposed_direction);

            if let Some(max) = movement_max {
                if proposed_position.x < 0
                    || proposed_position.x >= max.width as i32
                    || proposed_position.y < 0
                    || proposed_position.y >= max.height as i32
                {
                    continue;
                }
            }

            if blockers.iter().any(|blocker| *blocker == proposed_position) {
                continue;
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
                .insert(level::LevelComponent)
                .insert(*direction)
                .insert(*position);

            *direction = proposed_direction;
            *position = proposed_position;
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

fn check_win(
    mut commands: Commands,
    dragons: Query<(&grid::GridPosition, &Direction), With<DragonHead>>,
) {
    let dragons_opposite = dragons
        .iter_combinations::<2>()
        .any(|[a, b]| a.0.apply_direction(*a.1) == *b.0 && a.1.opposite() == *b.1);

    if dragons_opposite {
        commands.insert_resource(level::WinTimer(Timer::from_seconds(0.5, false)));
    }
}

impl<State: StateData> AssetProvider<State> for DragonPlugin {
    fn provide(&self, state: LoadingState<State>) -> LoadingState<State> {
        state.with_collection::<DragonAssets>()
    }
}

impl Plugin for DragonPlugin {
    fn build(&self, app: &mut App) {
        app.register_loadable::<DragonBundle>()
            .add_stage_before(
                grid::GridStage,
                "EntityProcessing",
                SystemStage::parallel()
                    .with_system(rotate_dragons)
                    .with_system(
                        check_win
                            .run_unless_resource_exists::<level::WinTimer>()
                            .run_in_state(State::InLevel),
                    ),
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
