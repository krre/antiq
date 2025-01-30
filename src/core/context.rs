use std::rc::Rc;

use crate::platform::{self, PlatformApplication};

pub struct Context {
    pub(crate) platform_context: Rc<dyn platform::PlatformContext>,
}

impl Context {
    pub(crate) fn new(app: &dyn PlatformApplication) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            platform_context: platform::Context::new(app)?.into(),
        })
    }
}
