use bevy::prelude::*;

use crate::util::prelude::*;

use super::loadable::TileBundle;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.register_loadable::<TileBundle>();
    }
}
