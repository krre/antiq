use std::{error::Error, sync::OnceLock};
use thiserror::Error;
use winit::application::ApplicationHandler;

use super::Window;

static INITED: OnceLock<bool> = OnceLock::new();

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Application already inited")]
    AlreadyInited,
}

pub struct Application {}

impl Application {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        if INITED.set(true).is_err() {
            return Err(Box::new(ApplicationError::AlreadyInited))?;
        }

        Ok(Self {})
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let event_loop = winit::event_loop::EventLoop::new()?;
        let _ = event_loop.run_app(self);
        Ok(())
    }

    pub fn create_window(&self) -> Window {
        Window::new()
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // todo!()
    }
}
