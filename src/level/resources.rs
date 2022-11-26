use bevy::{prelude::Resource, time::Timer};

#[derive(Clone, Debug, Resource)]
pub struct WinTimer(pub Timer);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Resource)]
pub struct CurrentLevel(pub usize);
