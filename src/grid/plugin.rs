use crate::{stage::EntityFinalisationStage, util::prelude::*};

use bevy::prelude::*;

use super::{loadable::GridBundle, systems::align_to_grid};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_loadable::<GridBundle>()
            .add_system_to_stage(EntityFinalisationStage, align_to_grid);
    }
}
