use std::any::Any;
use std::rc::Rc;

use crate::core::Result;
use crate::platform::PlatformApplication;

pub struct Application {}

impl Application {
    pub fn new() -> Result<Rc<dyn PlatformApplication>> {
        Ok(Rc::new(Self {}))
    }
}

impl PlatformApplication for Application {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
