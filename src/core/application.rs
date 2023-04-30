use crate::gfx::Engine;

use super::window::Settings;
use super::Window;
use super::{window, Position};
use once_cell::sync::OnceCell;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowId;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

static ORGANIZATION: OnceCell<String> = OnceCell::new();
static NAME: OnceCell<String> = OnceCell::new();

pub struct Application {
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, RefCell<Window>>,
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

    pub fn set_organization(organization: &str) {
        ORGANIZATION.set(organization.to_string()).unwrap();
    }

    pub fn organization() -> Option<String> {
        ORGANIZATION.get().cloned()
    }

    pub fn set_name(name: &str) {
        NAME.set(name.to_string()).unwrap();
    }

    pub fn name() -> Option<String> {
        NAME.get().cloned()
    }

    pub fn file_name() -> Option<String> {
        std::env::current_exe()
            .ok()?
            .file_name()?
            .to_str()?
            .to_owned()
            .into()
    }

    pub(crate) fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub(crate) fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn create_window(&mut self, settings: Settings) -> RefMut<Window> {
        let w = RefCell::new(Window::new(self, settings));
        let id = w.borrow().id();
        self.windows.insert(id.winit_id(), w);
        self.window_mut(id)
    }

    pub fn window_ref(&self, id: window::Id) -> Ref<Window> {
        self.windows.get(&id.winit_id()).unwrap().borrow()
    }

    pub fn window_mut(&self, id: window::Id) -> RefMut<Window> {
        self.windows.get(&id.winit_id()).unwrap().borrow_mut()
    }

    pub fn run(&mut self) {
        self.event_loop.run_return(|event, _, control_flow| {
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
                        .borrow_mut()
                        .resize(self.engine.gpu().device(), size);
                }

                Event::RedrawRequested(window_id) => {
                    self.windows.get(&window_id).unwrap().borrow().draw();

                    self.windows
                        .get(&window_id)
                        .unwrap()
                        .borrow()
                        .render(self.engine.gpu());
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    self.windows
                        .get(&window_id)
                        .unwrap()
                        .borrow_mut()
                        .set_visible(false);
                    self.windows.remove(&window_id);

                    if self.windows.len() == 0 {
                        control_flow.set_exit();
                    }
                }

                Event::WindowEvent {
                    event: WindowEvent::Moved(pos),
                    window_id,
                } => {
                    self.windows.get(&window_id).map(|w| {
                        w.borrow_mut()
                            .set_cache_position(Position::new(pos.x, pos.y))
                    });
                }

                _ => (),
            }
        });
    }
}
