use boba::prelude::*;

pub struct FpsPrinter;

register_pearl_stages!(FpsPrinter: BobaUpdate);

impl PearlStage<BobaUpdate> for FpsPrinter {
    fn update(&mut self, delta: &f32, _: &mut BobaResources) -> BobaResult {
        println!("FPS: {:.0}", 1. / delta);
        Ok(())
    }
}

fn main() {
    let mut app = Bobarista::<TaroMilkTea>::default();
    app.registry.add(&Pearl::wrap(FpsPrinter));
    app.run().unwrap();
}
