use boba_core::*;
use milk_tea_runner::events::MilkTeaResize;

use crate::TaroRenderer;

pub struct ResizeController;

impl ControllerData for ResizeController {}

impl ControllerStage<BobaEvent<MilkTeaResize>> for ResizeController {
    fn update<'a>(
        &mut self,
        data: &mut BobaEvent<MilkTeaResize>,
        resources: &mut boba_core::BobaResources,
    ) {
        let Some(renderer) = resources.get_mut::<TaroRenderer>() else {
                return;
            };

        renderer.resize(*data.data().size());
    }
}