use crate::renderer::Renderer;
use std::{
    fmt::Display,
    rc::{Rc, Weak},
    sync::OnceLock,
};

static APP_LOCK: OnceLock<()> = OnceLock::new();

pub struct Application {
    name: String,
    organization: String,
    renderer: Rc<Renderer>,
}

pub struct ApplicationBuilder {
    name: String,
    organization: String,
}

#[derive(Debug)]
pub enum ApplicationError {
    AlreadyExists,
    Other(Box<dyn std::error::Error>),
}

impl Application {
    pub fn new() -> std::result::Result<Self, ApplicationError> {
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

    pub fn renderer(&self) -> Weak<Renderer> {
        Rc::downgrade(&self.renderer)
    }

    pub fn run(&self) -> crate::core::Result<()> {
        Ok(())
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

    pub fn build(self) -> std::result::Result<Application, ApplicationError> {
        if APP_LOCK.set(()).is_err() {
            return Err(ApplicationError::AlreadyExists);
        }

        let application = Application {
            name: self.name,
            organization: self.organization,
            renderer: Rc::new(Renderer::new()),
        };

        Ok(application)
    }
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::AlreadyExists => write!(f, "application instance already exists"),
            ApplicationError::Other(e) => write!(f, "other application error: {}", e),
        }
    }
}

impl std::error::Error for ApplicationError {}
