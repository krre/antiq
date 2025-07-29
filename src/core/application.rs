use std::rc::Rc;

use wasm_bindgen::JsValue;

use crate::core::context::Context;

pub trait Application: Default {
    fn run(&self);
}

pub struct ApplicationBackend<App: Application> {
    app: App,
    context: Rc<Context>,
}

impl<App: Application> ApplicationBackend<App> {
    pub fn new() -> Result<Self, JsValue> {
        let context = Rc::new(Context::new());
        let app = App::default();

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
