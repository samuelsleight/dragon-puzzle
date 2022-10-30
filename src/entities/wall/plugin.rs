use bevy::prelude::*;

use crate::util::prelude::*;

use super::loadable::WallBundle;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.register_loadable::<WallBundle>();
    }
}
