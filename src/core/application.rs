use super::{error::ApplicationError, Context, EventLoop};
use crate::{platform, renderer::Renderer};
use std::{rc::Rc, sync::OnceLock};

static APP_LOCK: OnceLock<()> = OnceLock::new();

pub struct Application {
    name: String,
    organization: String,
    event_loop: EventLoop,
    renderer: Renderer,
    context: Rc<Context>,
    platform_application: Box<dyn platform::PlatformApplication>,
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

    pub fn context(&self) -> Rc<Context> {
        self.context.clone()
    }

    pub(crate) fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn run(&self) {
        self.event_loop.run();
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

        let context = Rc::new(Context::new()?);

        Ok(Application {
            name: self.name,
            organization: self.organization,
            event_loop: EventLoop::new(context.clone())?,
            renderer: Renderer::new(),
            context,
            platform_application: platform::Application::new()?,
        })
    }
}
