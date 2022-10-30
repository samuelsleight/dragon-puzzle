use bevy::prelude::Component;

use crate::Direction;

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GridSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct GridScale {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl GridScale {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn new_square(scale: f32) -> Self {
        Self::new(scale, scale)
    }
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn apply_direction(&self, direction: Direction) -> Self {
        let (dx, dy) = direction.delta();

        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}
