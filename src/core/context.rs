use crate::platform;

pub struct Context {
    platform_context: Box<dyn platform::PlatformContext>,
}

impl Context {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            platform_context: platform::Context::new()?,
        })
    }
}
