use bevy::prelude::Component;

use crate::grid::GridPosition;

#[derive(Component)]
pub struct MovementManager;

#[derive(Component)]
pub struct Blocker;

#[derive(Component, Default)]
pub struct Movement(pub Option<GridPosition>);
