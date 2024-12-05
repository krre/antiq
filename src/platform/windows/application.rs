use crate::platform::PlatformApplication;

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }
}

impl PlatformApplication for Application {}
