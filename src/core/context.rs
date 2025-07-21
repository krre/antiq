use std::rc::Rc;

use crate::Renderer;

pub struct Context {
    renderer: Rc<Renderer>,
}

impl Context {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            renderer: Rc::new(renderer),
        }
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
}
