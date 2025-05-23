use crate::core::Result;
use crate::platform::PlatformWindow;

pub struct Window {}

impl Window {
    pub fn new() -> Result<Box<dyn PlatformWindow>> {
        Ok(Box::new(Self {}))
    }
}
