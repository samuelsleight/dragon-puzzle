use bevy::prelude::*;

use super::components::{GridPosition, GridScale, GridSize};

fn convert_coordinate(grid_size: u32, grid_scale: f32, position: i32) -> f32 {
    let max_pixels = grid_size as f32 * grid_scale;
    let this_pixels = position as f32 * grid_scale;
    (0.0 - (max_pixels / 2.0) + this_pixels) + (grid_scale / 2.0)
}

pub fn align_to_grid(
    grid_query: Query<(&GridSize, &GridScale)>,
    mut entity_query: Query<(&GridPosition, &mut Transform), Changed<GridPosition>>,
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
