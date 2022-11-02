use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    action::Action,
    direction::Direction,
    grid::{GridPosition, GridSize},
};

use super::components::{Blocker, Movement, MovementManager};

pub fn process_movement(
    actions_query: Query<&ActionState<Action>, With<MovementManager>>,
    grid_query: Query<&GridSize>,
    blockers_query: Query<&GridPosition, With<Blocker>>,
    mut movement_query: Query<(&GridPosition, &mut Direction, &mut Movement)>,
) {
    for action in actions_query.single().get_just_pressed() {
        let action = match action.movement() {
            Some(action) => action,
            _ => continue,
        };

        let movement_max = grid_query.get_single().ok();

        for (position, mut direction, mut movement) in movement_query.iter_mut() {
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

            if blockers_query
                .iter()
                .any(|blocker| *blocker == proposed_position)
            {
                continue;
            }

            *direction = proposed_direction;
            movement.0 = Some(proposed_position);
        }
    }
}

pub fn finish_movement(mut query: Query<(&mut GridPosition, &mut Movement), Changed<Movement>>) {
    for (mut position, mut movement) in query.iter_mut() {
        if let Some(proposed_position) = movement.0.take() {
            *position = proposed_position;
        }
    }
}
