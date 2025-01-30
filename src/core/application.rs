use super::{
    error::ApplicationError,
    event::{EventHandler, WindowAction, WindowEvent},
    Context, EventLoop,
};
use crate::{platform, renderer::Renderer};
use std::{rc::Rc, sync::OnceLock};

static APP_LOCK: OnceLock<()> = OnceLock::new();

pub struct Application {
    name: String,
    organization: String,
    event_loop: Rc<EventLoop>,
    context: Rc<Context>,
    platform_application: Box<dyn platform::PlatformApplication>,
    quit_on_last_window_closed: bool,
}

pub struct ApplicationBuilder {
    name: String,
    organization: String,
    quit_on_last_window_closed: bool,
}

struct ApplicationEventHandler<'app> {
    application: &'app Application,
    event_loop: Rc<EventLoop>,
}

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
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

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let event_handler = ApplicationEventHandler {
            application: self,
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
            quit_on_last_window_closed: true,
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

    pub fn quit_on_last_window_closed(mut self, quit: bool) -> Self {
        self.quit_on_last_window_closed = quit;
        self
    }

    pub fn build(self) -> Result<Application, ApplicationError> {
        if let Err(_) = APP_LOCK.set(()) {
            return Err(ApplicationError::AlreadyExists);
        }

        let platform_application =
            platform::Application::new().map_err(|e| ApplicationError::Other(e))?;
        let renderer = Rc::new(Renderer::new());

        let context = Rc::new(
            Context::new(platform_application.as_ref(), renderer.clone())
                .map_err(|e| ApplicationError::Other(e))?,
        );

        let event_loop =
            Rc::new(EventLoop::new(context.clone()).map_err(|e| ApplicationError::Other(e))?);

        Ok(Application {
            name: self.name,
            organization: self.organization,
            context,
            event_loop,
            platform_application,
            quit_on_last_window_closed: self.quit_on_last_window_closed,
        })
    }
}

impl<'app> EventHandler for ApplicationEventHandler<'app> {
    fn window_event(&self, event: WindowEvent) {
        match event.action {
            WindowAction::Redraw => {
                self.application.context().window_manager().render(event.id);
            }
            WindowAction::Close => {
                self.application.context.window_manager().remove(event.id);

                if self.application.context.window_manager().count() == 0
                    && self.application.quit_on_last_window_closed
                {
                    self.event_loop.quit();
                }
            }
            WindowAction::Resize(size) => {
                self.application
                    .context
                    .window_manager()
                    .resize(event.id, size);
            }
            WindowAction::Move(pos) => {
                self.application
                    .context
                    .window_manager()
                    .move_to(event.id, pos);
            }
        }
    }
}
