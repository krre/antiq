use std::rc::Rc;

use wasm_bindgen::JsValue;

use crate::{core::context::Context, renderer::Renderer};

pub trait Application {
    fn new(context: Rc<Context>) -> Self;

    fn run(&self);
}

pub struct ApplicationBackend<App: Application> {
    app: App,
    context: Rc<Context>,
}

impl<App: Application> ApplicationBackend<App> {
    pub fn new() -> Result<Self, JsValue> {
        let renderer = Renderer::new();
        let context = Rc::new(Context::new(renderer));
        let app = App::new(context.clone());

        let backend = Self { app, context };

        Ok(backend)
    }

    pub fn run(&self) {
        self.app.run();
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
}
