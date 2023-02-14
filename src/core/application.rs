use crate::gfx::Engine;

use super::{window, Window};
use std::collections::HashMap;
use std::ops::Deref;
use winit::window::WindowId;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

pub struct Application {
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, Box<window::Window>>,
    engine: Engine,
}

impl Application {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            windows: HashMap::new(),
            engine: Engine::new(),
        }
    }

    pub(crate) fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub(crate) fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn create_window(&mut self) -> &Window {
        let window = Box::new(Window::new(self));
        let id = window.id().winit_id();
        self.windows.insert(id, window);
        self.windows.get(&id).unwrap().deref()
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    self.windows
                        .get(&window_id)
                        .unwrap()
                        .resize(self.engine.gpu().device(), size);
                }

                Event::RedrawRequested(window_id) => {
                    self.windows
                        .get(&window_id)
                        .unwrap()
                        .render(self.engine.gpu());
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    self.windows.get(&window_id).unwrap().set_visible(false);
                    self.windows.remove(&window_id);

                    if self.windows.len() == 0 {
                        control_flow.set_exit();
                    }
                }

                _ => (),
            }
        });
    }

    pub fn name() -> Option<String> {
        std::env::current_exe()
            .ok()?
            .file_name()?
            .to_str()?
            .to_owned()
            .into()
    }
}
