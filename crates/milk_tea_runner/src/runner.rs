use boba_core::BobaRunner;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::MilkTeaWindows;

#[derive(Default)]
pub struct MilkTeaRunner {}

impl BobaRunner for MilkTeaRunner {
    fn run(&mut self, mut app: boba_core::BobaApp) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let main_id = window.id();

        app.resources().add(MilkTeaWindows::new(window));

        app.execute_startup_stages();

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == main_id => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    // WindowEvent::Resized(physical_size) => app
                    //     .resources()
                    //     .get_mut::<MilkTeaRender>()
                    //     .expect("Renderer was not in resources")
                    //     .resize(*physical_size),
                    // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => app
                    //     .resources()
                    //     .get_mut::<MilkTeaRender>()
                    //     .expect("Renderer was not in resources")
                    //     .resize(**new_inner_size),
                    _ => (),
                },
                Event::MainEventsCleared => {
                    app.execute_stages();
                }
                _ => (),
            }
        });
    }
}
