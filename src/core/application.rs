use super::{
    error::ApplicationError,
    event::{EventHandler, WindowAction, WindowEvent},
    Context, EventLoop,
};
use crate::{
    core::event::{ApplicationAction, ApplicationEvent},
    platform,
    renderer::Renderer,
};
use std::{rc::Rc, sync::OnceLock};

static APP_LOCK: OnceLock<()> = OnceLock::new();

pub struct Application {
    name: String,
    organization: String,
    event_loop: Rc<EventLoop>,
    renderer: Rc<Renderer>,
    context: Rc<Context>,
    platform_application: Box<dyn platform::PlatformApplication>,
}

pub struct ApplicationBuilder {
    name: String,
    organization: String,
}

struct ApplicationEventHandler {
    context: Rc<Context>,
    event_loop: Rc<EventLoop>,
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

    pub fn context(&self) -> Rc<Context> {
        self.context.clone()
    }

    pub fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let event_handler = ApplicationEventHandler {
            context: self.context.clone(),
            event_loop: self.event_loop.clone(),
        };
        self.event_loop.run(&event_handler)
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
        if let Err(_) = APP_LOCK.set(()) {
            return Err(Box::new(ApplicationError::AlreadyExists));
        }

        let platform_application = platform::Application::new()?;
        let renderer = Rc::new(Renderer::new());
        let context = Rc::new(Context::new(
            platform_application.as_ref(),
            renderer.clone(),
        )?);
        let event_loop = Rc::new(EventLoop::new(context.clone())?);

        Ok(Application {
            name: self.name,
            organization: self.organization,
            context,
            event_loop,
            renderer,
            platform_application,
        })
    }
}

impl EventHandler for ApplicationEventHandler {
    fn window_event(&self, event: WindowEvent) {
        match event.action {
            WindowAction::Redraw => {
                self.context.window_manager().render_window(event.id);
            }
            WindowAction::Close => {
                self.context.window_manager().remove_window(event.id);

                if self.context.window_manager().window_count() == 0 {
                    self.event_loop.send_event(Box::new(ApplicationEvent {
                        action: ApplicationAction::Quit,
                    }));
                }
            }
            WindowAction::Resize(size) => {
                self.context
                    .window_manager()
                    .update_window_size(event.id, size);
            }
            WindowAction::Move(pos) => {
                self.context
                    .window_manager()
                    .update_window_position(event.id, pos);
            }
        }
    }
}
