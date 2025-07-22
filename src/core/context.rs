use std::rc::Rc;

use crate::{Renderer, Window};

pub struct Context {
    renderer: Rc<Renderer>,
    window: Rc<Window>,
}

impl Context {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            renderer: Rc::new(renderer),
            window: Rc::new(Window::new()),
        }
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
