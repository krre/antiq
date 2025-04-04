use crate::core::Result;
use crate::platform::PlatformApplication;

pub struct Application {}

impl Application {
    pub fn new() -> Result<Box<dyn PlatformApplication>> {
        unimplemented!()
    }
}

impl PlatformApplication for Application {}
