use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    platform::{self, PlatformApplication},
    renderer::Renderer,
};

use super::{Window, WindowId};

pub struct Context {
    pub(crate) platform_context: Rc<dyn platform::PlatformContext>,
    renderer: Rc<Renderer>,
    windows: RefCell<HashMap<WindowId, Rc<Window>>>,
}

impl Context {
    pub(crate) fn new(
        app: &dyn PlatformApplication,
        renderer: Rc<Renderer>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            platform_context: platform::Context::new(app)?.into(),
            renderer,
            windows: RefCell::new(HashMap::new()),
        })
    }

    pub fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }

    pub(crate) fn add_window(&self, id: WindowId, window: Rc<Window>) {
        self.windows.borrow_mut().insert(id, window);
    }

    pub(crate) fn remove_window(&self, id: WindowId) {
        self.windows.borrow_mut().remove(&id);
    }

    pub(crate) fn render_window(&self, id: WindowId) {
        self.windows.borrow().get(&id).unwrap().render();
    }
}
