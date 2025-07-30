use std::rc::Rc;

use wasm_bindgen::JsValue;

use crate::{core::context::Context, ui::Ui3d};

pub trait Application: Default {
    fn build_ui(&self) -> Ui3d;
}

pub struct ApplicationBackend<App: Application> {
    _app: App,
    _ui: Ui3d,
    context: Rc<Context>,
}

impl<App: Application> ApplicationBackend<App> {
    pub fn new() -> Result<Self, JsValue> {
        let context = Rc::new(Context::new());
        let app = App::default();
        let ui = app.build_ui();

        let backend = Self {
            _app: app,
            _ui: ui,
            context,
        };

        Ok(backend)
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
}
