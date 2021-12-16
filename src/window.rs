//use penguin_config::app_config::WindowConfig;
use crate::config::WindowConfig;

pub mod window_events {
    pub use winit::event::*;
}


// window resource
pub struct Window {
    pub handle: winit::window::Window,
    pub dimensions: winit::dpi::PhysicalSize<u32>,
    pub logger_level: log::LevelFilter,
}


impl Window {
    pub(crate) fn new(
        event_loop: &winit::event_loop::EventLoop<()>,
        dimensions: WindowConfig,
        logger_level: log::LevelFilter,
    ) -> Self {
        let dimensions = winit::dpi::PhysicalSize::new(dimensions.width, dimensions.height);

        Self {
            handle: winit::window::WindowBuilder::new()
                .with_title("penguin engine")
                .with_inner_size(dimensions)
                .build(&event_loop)
                .expect("Window could not be created"),
            dimensions,
            logger_level,
        }
    }
}
impl std::ops::Deref for Window {
    type Target = winit::window::Window;
    fn deref(&self) -> &Self::Target { &self.handle }
}

