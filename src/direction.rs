use bevy::prelude::Component;
use serde::Deserialize;

use crate::action::MovementAction;

#[derive(Deserialize, Component, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn process_action(self, action: MovementAction) -> Self {
        match action {
            MovementAction::Forwards => self,
            MovementAction::TurnLeft => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            MovementAction::TurnRight => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
        }
    }

    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
