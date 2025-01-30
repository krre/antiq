use std::rc::Rc;

use crate::{
    platform::{self, PlatformApplication},
    renderer::Renderer,
};

use super::window_manager::WindowManager;

pub struct Context {
    pub(crate) platform_context: Rc<dyn platform::PlatformContext>,
    renderer: Rc<Renderer>,
    window_manager: WindowManager,
}

impl Context {
    pub(crate) fn new(
        app: &dyn PlatformApplication,
        renderer: Rc<Renderer>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            platform_context: platform::Context::new(app)?.into(),
            renderer,
            window_manager: WindowManager::new(),
        })
    }

    pub fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }

    pub(crate) fn window_manager(&self) -> &WindowManager {
        &self.window_manager
    }
}
