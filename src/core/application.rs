use std::rc::Rc;

use crate::renderer::Renderer;

use super::window::WindowSettings;
use super::{AppContext, EventLoop};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoopBuilder};
use winit::window::WindowId;

pub struct Application {
    name: String,
    organization: String,
    event_loop: EventLoop,
    renderer: Renderer,
    context: Rc<AppContext>,
    on_run: Option<Box<dyn Fn(Rc<AppContext>)>>,
}

pub struct ApplicationBuilder {
    name: String,
    organization: String,
}

#[derive(Debug)]
pub enum UserEvent {
    CreateWindow(WindowSettings),
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

    pub fn context(&self) -> Rc<AppContext> {
        self.context.clone()
    }

    pub(crate) fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    // pub fn run<F>(&mut self, on_run: F)
    // where
    //     F: 'static + Fn(Rc<AppContext>),
    // {
    //     self.on_run = Some(Box::new(on_run));

    //     let mut builder: EventLoopBuilder<UserEvent> =
    //         winit::event_loop::EventLoop::with_user_event();
    //     let event_loop = builder.build().unwrap();
    //     event_loop.set_control_flow(ControlFlow::Wait);
    //     self.context = Some(Rc::new(AppContext::new(event_loop.create_proxy())));

    //     event_loop.run_app(self).unwrap();
    // }

    pub fn run(&self) {
        self.event_loop.run();
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(on_run) = &self.on_run {
            // on_run(self.context.as_ref().unwrap().clone());
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
                // self.windows
                //     .get(&window_id)
                //     .unwrap()
                //     .borrow_mut()
                //     .resize(self.gfx_engine.gpu().device(), size);
            }

            WindowEvent::RedrawRequested => {
                // self.windows.get(&window_id).unwrap().borrow().build();

                // self.windows
                //     .get(&window_id)
                //     .unwrap()
                //     .borrow()
                //     .render(self.gfx_engine.gpu());
            }

            WindowEvent::CloseRequested => {
                // self.context
                //     .as_ref()
                //     .unwrap()
                //     .windows()
                //     .borrow_mut()
                //     .remove(&window_id);

                // if self.context.as_ref().unwrap().windows().borrow().len() == 0 {
                //     event_loop.exit();
                // }
            }

            _ => (),
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::CreateWindow(settings) => {
                println!("user event creating window");
                let mut window_attributes = winit::window::Window::default_attributes()
                    .with_title(&settings.title)
                    .with_visible(settings.visible)
                    .with_maximized(settings.maximized);

                // let position = if let Some(position) = settings.position {
                //     window_attributes = window_attributes.with_position(
                //         winit::dpi::PhysicalPosition::new(position.x(), position.y()),
                //     );
                //     position
                // } else {
                //     Position::new(200, 200)
                // };

                if let Some(size) = settings.size {
                    window_attributes = window_attributes
                        .with_inner_size(winit::dpi::PhysicalSize::new(size.width, size.height));
                }

                let window = event_loop.create_window(window_attributes).unwrap();
                let id = window.id();
                // self.context
                //     .as_ref()
                //     .unwrap()
                //     .windows()
                //     .borrow_mut()
                //     .insert(id, window);
            }
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
        Ok(Application {
            name: self.name,
            organization: self.organization,
            event_loop: EventLoop::new(),
            renderer: Renderer::new(),
            context: Rc::new(AppContext::new()),
            on_run: None,
        })
    }
}
