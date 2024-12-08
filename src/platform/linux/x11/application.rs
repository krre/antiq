use crate::platform::PlatformApplication;

pub struct Application {}

impl Application {
    pub fn new() -> Result<Box<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self {}))
    }
}

impl PlatformApplication for Application {}
