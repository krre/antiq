use crate::gfx::Engine;

use super::window;
use super::Window;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use winit::window::WindowId;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

pub struct Application {
    event_loop: EventLoop<()>,
    windows: RefCell<HashMap<WindowId, Rc<Window>>>,
    engine: Engine,
}

impl Application {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            windows: RefCell::new(HashMap::new()),
            engine: Engine::new(),
        }
    }

    pub(crate) fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub(crate) fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn create_window(&self) -> Rc<Window> {
        let w = Rc::new(Window::new(self));
        let id = w.id();
        self.windows.borrow_mut().insert(id.winit_id(), w);
        self.window(id)
    }

    pub fn window(&self, id: window::Id) -> Rc<Window> {
        self.windows.borrow().get(&id.winit_id()).unwrap().clone()
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    self.windows
                        .borrow()
                        .get(&window_id)
                        .unwrap()
                        .resize(self.engine.gpu().device(), size);
                }

                Event::RedrawRequested(window_id) => {
                    self.windows
                        .borrow()
                        .get(&window_id)
                        .unwrap()
                        .render(self.engine.gpu());
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    self.windows
                        .borrow()
                        .get(&window_id)
                        .unwrap()
                        .set_visible(false);
                    self.windows.borrow_mut().remove(&window_id);

                    if self.windows.borrow().len() == 0 {
                        control_flow.set_exit();
                    }
                }

                Event::WindowEvent {
                    event: WindowEvent::Moved(pos),
                    window_id,
                } => {
                    self.windows
                        .borrow()
                        .get(&window_id)
                        .map(|w| w.set_cache_position(pos));
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
