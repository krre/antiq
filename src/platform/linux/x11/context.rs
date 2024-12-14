use crate::platform::PlatformContext;

pub struct Context {}

impl Context {
    pub fn new() -> Result<Box<dyn PlatformContext>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self {}))
    }
}

impl PlatformContext for Context {}
