use bevy::prelude::StageLabel;

pub struct InputHandlingStage;
pub struct EntityProcessingStage;
pub struct EntityFinalisationStage;

impl StageLabel for InputHandlingStage {
    fn as_str(&self) -> &'static str {
        "InputHandling"
    }
}

impl StageLabel for EntityProcessingStage {
    fn as_str(&self) -> &'static str {
        "EntityProcessing"
    }
}

impl StageLabel for EntityFinalisationStage {
    fn as_str(&self) -> &'static str {
        "EntityFinalisation"
    }
}
