use bevy::prelude::Component;

use crate::grid::GridPosition;

#[derive(Component)]
pub struct DragonHead;

#[derive(Component, Default)]
pub struct Movement(pub Option<GridPosition>);
