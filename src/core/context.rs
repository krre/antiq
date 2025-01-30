use std::rc::Rc;

use crate::{
    platform::{self, PlatformApplication},
    renderer::Renderer,
};

pub struct Context {
    pub(crate) platform_context: Rc<dyn platform::PlatformContext>,
    renderer: Rc<Renderer>,
}

impl Context {
    pub(crate) fn new(
        app: &dyn PlatformApplication,
        renderer: Rc<Renderer>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            platform_context: platform::Context::new(app)?.into(),
            renderer,
        })
    }

    pub fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }
}
