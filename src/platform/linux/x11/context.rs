use std::any::Any;

use crate::platform::{PlatformApplication, PlatformContext};

pub struct Context {}

impl Context {
    pub fn new(
        app: &dyn PlatformApplication,
    ) -> Result<Box<dyn PlatformContext>, Box<dyn std::error::Error>> {
        Ok(Box::new(Self {}))
    }
}

impl PlatformContext for Context {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
