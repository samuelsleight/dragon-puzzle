use crate::util::prelude::*;

use bevy::prelude::*;

use super::{loadable::GridBundle, systems::align_to_grid};

pub struct GridPlugin;

pub struct GridStage;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_loadable::<GridBundle>().add_stage_before(
            CoreStage::PostUpdate,
            GridStage,
            SystemStage::parallel().with_system(align_to_grid),
        );
    }
}

impl StageLabel for GridStage {
    fn as_str(&self) -> &'static str {
        "GridStage"
    }
}
