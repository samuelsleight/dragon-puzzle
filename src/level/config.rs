use bevy::reflect::TypeUuid;
use serde::Deserialize;

use crate::direction::Direction;

#[derive(Deserialize, Clone, Copy)]
pub struct DragonConfig {
    pub position: [i32; 2],
    pub direction: Direction,
}

#[derive(Deserialize, TypeUuid, Clone)]
#[uuid = "8d84e066-5bad-49f1-85d1-60788779f1d5"]
pub struct LevelConfig {
    pub size: [u32; 2],
    pub dragons: Vec<DragonConfig>,
}
