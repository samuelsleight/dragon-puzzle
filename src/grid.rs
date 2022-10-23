use bevy::prelude::*;

use crate::Direction;

#[derive(Bundle, Clone, Copy)]
pub struct GridBundle {
    pub size: GridSize,
    pub scale: GridScale,
}

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

pub struct GridPlugin;

pub struct GridStage;

impl GridPosition {
    pub fn apply_direction(&self, direction: Direction) -> Self {
        let (dx, dy) = direction.delta();

        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

fn convert_coordinate(grid_size: u32, grid_scale: f32, position: i32) -> f32 {
    let max_pixels = grid_size as f32 * grid_scale;
    let this_pixels = position as f32 * grid_scale;
    (0.0 - (max_pixels / 2.0) + this_pixels) + (grid_scale / 2.0)
}

fn align_to_grid(
    grid_query: Query<(&GridSize, &GridScale)>,
    mut entity_query: Query<(&GridPosition, &mut Transform)>,
) {
    let (size, scale) = match grid_query.get_single() {
        Ok(result) => result,
        Err(_) => return,
    };

    for (position, mut transform) in entity_query.iter_mut() {
        transform.translation = Vec3::new(
            convert_coordinate(size.width, scale.width, position.x),
            convert_coordinate(size.height, scale.height, position.y),
            0.0,
        );
    }
}

impl GridSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn new_square(size: u32) -> Self {
        Self::new(size, size)
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

impl StageLabel for GridStage {
    fn as_str(&self) -> &'static str {
        "GridStage"
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage(
            GridStage,
            SystemStage::parallel().with_system(align_to_grid),
        );
    }
}
