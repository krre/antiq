use std::rc::Rc;

use crate::{Renderer, Window};

pub struct Context {
    renderer: Rc<Renderer>,
    window: Rc<Window>,
}

impl Context {
    pub fn new() -> Self {
        let window = Window::new();
        let renderer = Renderer::new(&window);

        Self {
            renderer: Rc::new(renderer),
            window: Rc::new(window),
        }
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
