use crate::platform::PlatformApplication;

pub struct Application {}

impl Application {
    pub fn new() -> Result<Box<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

impl PlatformApplication for Application {}
