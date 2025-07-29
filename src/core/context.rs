use std::rc::Rc;

use crate::{Renderer, Window};

pub struct Context {
    renderer: Rc<Renderer>,
    window: Rc<Window>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            renderer: Rc::new(Renderer::new()),
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
