use std::f32::consts::PI;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    action::Action,
    entities::wall::Blocker,
    grid::{GridPosition, GridSize},
    level::{LevelComponent, WinTimer},
    Direction,
};

use super::{assets::DragonAssets, components::DragonHead};

pub fn dragon_movement(
    mut commands: Commands,
    assets: Res<DragonAssets>,
    grid_query: Query<&GridSize>,
    mut set: ParamSet<(
        Query<(&ActionState<Action>, &mut Direction, &mut GridPosition), With<DragonHead>>,
        Query<&GridPosition, With<Blocker>>,
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
                .insert(LevelComponent)
                .insert(*direction)
                .insert(*position);

            *direction = proposed_direction;
            *position = proposed_position;
        }
    }
}

pub fn rotate_dragons(mut q: Query<(&Direction, &mut Transform)>) {
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

pub fn check_win(
    mut commands: Commands,
    dragons: Query<(&GridPosition, &Direction), With<DragonHead>>,
) {
    let dragons_opposite = dragons
        .iter_combinations::<2>()
        .any(|[a, b]| a.0.apply_direction(*a.1) == *b.0 && a.1.opposite() == *b.1);

    if dragons_opposite {
        commands.insert_resource(WinTimer(Timer::from_seconds(0.5, false)));
    }
}
