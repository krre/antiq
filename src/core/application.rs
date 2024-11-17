use crate::renderer::Engine;

use super::window::WindowSettings;
use super::{window, Position};
use super::{AppContext, Window};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

pub struct Application {
    name: String,
    organization: String,
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, RefCell<Window>>,
    gfx_engine: Engine,
    context: AppContext,
    on_run: Option<Box<dyn Fn(&mut AppContext)>>,
}

pub struct ApplicationBuilder {
    name: String,
    organization: String,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let builder = ApplicationBuilder::new();
        builder.build()
    }

    pub fn organization(&self) -> &str {
        &self.organization
    }

    pub fn name(&self) -> &str {
        &self.name
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

    pub(crate) fn gfx_engine(&self) -> &Engine {
        &self.gfx_engine
    }

    pub fn create_window(&mut self, settings: WindowSettings) -> RefMut<Window> {
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

    pub fn run<F>(&mut self, on_run: F)
    where
        F: 'static + Fn(&mut AppContext),
    {
        // self.event_loop.run_app(self);
        // self.event_loop.run_return(|event, _, control_flow| {
        //     control_flow.set_wait();
        // });
        self.on_run = Some(Box::new(on_run));
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(on_run) = &self.on_run {
            on_run(&mut self.context);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => {
                self.windows
                    .get(&window_id)
                    .unwrap()
                    .borrow_mut()
                    .resize(self.gfx_engine.gpu().device(), size);
            }

            WindowEvent::RedrawRequested => {
                self.windows.get(&window_id).unwrap().borrow().build();

                self.windows
                    .get(&window_id)
                    .unwrap()
                    .borrow()
                    .render(self.gfx_engine.gpu());
            }

            WindowEvent::CloseRequested => {
                self.windows
                    .get(&window_id)
                    .unwrap()
                    .borrow_mut()
                    .set_visible(false);
                self.windows.remove(&window_id);

                if self.windows.len() == 0 {
                    event_loop.exit();
                }
            }

            WindowEvent::Moved(pos) => {
                self.windows.get(&window_id).map(|w| {
                    w.borrow_mut()
                        .set_cache_position(Position::new(pos.x, pos.y))
                });
            }

            _ => (),
        }
    }
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self {
            name: Application::file_name().unwrap_or("Application".to_string()),
            organization: Application::file_name().unwrap_or("Antiq".to_string()),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn organization(mut self, organization: &str) -> Self {
        self.organization = organization.to_owned();
        self
    }

    pub fn build(self) -> Result<Application, Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Wait);

        Ok(Application {
            name: self.name,
            organization: self.organization,
            event_loop,
            windows: HashMap::new(),
            gfx_engine: Engine::new(),
            context: AppContext::new(),
            on_run: None,
        })
    }
}
