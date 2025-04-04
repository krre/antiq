use crate::core::Result;
use crate::platform::PlatformContext;

pub struct Context {}

impl Context {
    pub fn new() -> Result<Box<dyn PlatformContext>> {
        Ok(Box::new(Self {}))
    }
}
