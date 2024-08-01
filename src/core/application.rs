use std::sync::OnceLock;
use thiserror::Error;

use super::Window;

static INITED: OnceLock<bool> = OnceLock::new();

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Application already inited")]
    AlreadyInited,
}

pub struct Application {}

impl Application {
    pub fn new() -> Result<Self, ApplicationError> {
        if INITED.set(true).is_err() {
            return Err(ApplicationError::AlreadyInited);
        }

        Ok(Self {})
    }

    pub fn run(&self) {
        println!("Application started");
    }

    pub fn create_window(&self) -> Window {
        Window::new()
    }
}
