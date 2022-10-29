use crate::{
    storage::{controller_storage::ControllerStorage, stage_storage::StageStorage},
    BobaResources,
};

#[derive(Default)]
pub struct BobaApp {
    resources: BobaResources,
    controllers: ControllerStorage,
    stages: StageStorage,
}

impl BobaApp {
    pub fn stages(&mut self) -> &mut StageStorage {
        &mut self.stages
    }

    pub fn controllers(&mut self) -> &mut ControllerStorage {
        &mut self.controllers
    }

    pub fn update(&mut self) {
        for stage in self.stages.iter_mut() {
            stage.run(&mut self.controllers, &mut self.resources);
        }
    }
}
