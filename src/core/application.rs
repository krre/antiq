use wasm_bindgen::JsValue;

use crate::renderer::Renderer;

pub trait Application: Default {
    fn run(&self);
}

pub struct ApplicationBackend<App: Application> {
    app: App,
    renderer: Renderer,
}

impl<App: Application> ApplicationBackend<App> {
    pub fn new() -> Result<Self, JsValue> {
        let backend = Self {
            app: App::default(),
            renderer: Renderer::new(),
        };

        Ok(backend)
    }

    pub fn run(&self) {
        self.app.run();
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
}
