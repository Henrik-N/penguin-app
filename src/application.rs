use super::ecs::*;
use anyhow::*;

use winit::event::WindowEvent;
use winit::event::{ElementState, Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::config::AppConfig;


pub struct App {
    pub event_loop: Option<EventLoop<()>>,
    //
    world: World,
    resources: Resources,
    //
    startup_schedule: Schedule,
    run_schedule: Schedule,
    shutdown_schedule: Schedule,
}
impl App {
    pub fn builder(config: AppConfig) -> builder::AppBuilder {
        builder::AppBuilder::builder(config)
    }

    pub fn run(mut self) -> Result<()> {
        self.startup_schedule
            .execute(&mut self.world, &mut self.resources);

        let event_loop = self.event_loop.take().unwrap();
        self.run_loop(event_loop)
    }

    pub fn run_loop(mut self, event_loop: EventLoop<()>) -> Result<()> {
        event_loop.run(move |event, _, control_flow| {
            let _ = &self;
            *control_flow = ControlFlow::Poll;

            match event {
                // Window events ------------
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        log::info!("Close requested, closing...");
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        match (input.virtual_keycode, input.state) {
                            (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                *control_flow = ControlFlow::Exit
                            }
                            _ => {
                                let input_events_resource =
                                    self.resources.get_mut::<super::input::InputEvents>();
                                input_events_resource.unwrap().update(input);
                            }
                        }
                    }
                    _ => {}
                },
                // Other events ------------
                Event::MainEventsCleared => {
                    self.run_schedule
                        .execute(&mut self.world, &mut self.resources);
                }
                Event::RedrawRequested(_window_id) => {}
                Event::LoopDestroyed => {
                    self.shutdown_schedule
                        .execute(&mut self.world, &mut self.resources);
                }
                _ => (),
            }
        });
    }
}


pub mod builder {
    use super::*;
    use super::super::window;
    use super::super::input;
    use super::super::logger;

    use crate::config::AppConfig;

    pub struct AppBuilder {
        event_loop: EventLoop<()>,
        //
        world: World,
        resources: Resources,
        startup_steps: Vec<Step>,
        run_steps: Vec<Step>,
        shutdown_steps: Vec<Step>,
    }

    impl AppBuilder {
        pub fn builder(config: AppConfig) -> Self {

            logger::init_logger(config.logger_config.clone()).expect("Couldn't init logger");


            let event_loop = winit::event_loop::EventLoop::new();

            let mut resources = Resources::default();
            resources.insert(window::Window::new(
                &event_loop,
                config.clone().window_config,
                config.logger_config.debug_message_severity.clone()
            ));
            log::trace!("Window resource created");


            let builder = Self {
                event_loop,
                world: World::default(),
                resources,
                startup_steps: Vec::new(),
                run_steps: Vec::new(),
                shutdown_steps: Vec::new(),
            };

            builder.add_plugin(input::InputPlugin)
        }

        pub fn add_startup_steps(mut self, steps: Vec<Step>) -> Self {
            self.startup_steps.extend(steps.into_iter());
            self
        }

        pub fn add_run_steps(mut self, steps: Vec<Step>) -> Self {
            self.run_steps.extend(steps.into_iter());
            self
        }

        pub fn add_shutdown_steps(mut self, steps: Vec<Step>) -> Self {
            self.shutdown_steps.extend(steps.into_iter());
            self
        }

        #[allow(unused)]
        pub fn insert_resource<T: 'static>(mut self, resource: T) -> Self
        {
            self.resources.insert(resource);
            self
        }

        pub fn add_plugin<T: Plugin>(mut self, mut plugin: T) -> Self {
            let startup_steps = plugin.startup(&mut self.resources);
            self = self.add_startup_steps(startup_steps);
            self = self.add_run_steps(T::run());
            self = self.add_shutdown_steps(T::shutdown());
            self
        }

        pub fn run(self) -> Result<()> {
            let startup_schedule = Schedule::from(self.startup_steps);
            let run_schedule = Schedule::from(self.run_steps);
            let shutdown_schedule = Schedule::from(self.shutdown_steps);

            let app = super::App {
                event_loop: Some(self.event_loop),
                world: self.world,
                resources: self.resources,

                startup_schedule,
                run_schedule,
                shutdown_schedule,
            };

            app.run()
        }

        pub fn build(self) -> super::App {
            let startup_schedule = Schedule::from(self.startup_steps);
            let run_schedule = Schedule::from(self.run_steps);
            let shutdown_schedule = Schedule::from(self.shutdown_steps);

            super::App {
                event_loop: Some(self.event_loop),
                world: self.world,
                resources: self.resources,

                startup_schedule,
                run_schedule,
                shutdown_schedule,
            }
        }
    }
}
