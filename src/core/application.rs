use std::rc::Rc;

use wasm_bindgen::JsValue;

use crate::{core::context::Context, renderer::Renderer};

pub trait Application: Default {
    fn run(&self);
}

pub struct ApplicationBackend<App: Application> {
    app: App,
    context: Rc<Context>,
}

impl<App: Application> ApplicationBackend<App> {
    pub fn new() -> Result<Self, JsValue> {
        let renderer = Renderer::new();
        let context = Context::new(renderer);

        let backend = Self {
            app: App::default(),
            context: Rc::new(context),
        };

        Ok(backend)
    }

    pub fn run(&self) {
        self.app.run();
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
}
