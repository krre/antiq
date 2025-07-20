use wasm_bindgen::JsValue;

pub trait Application: Default {
    fn run(&self);
}

pub struct ApplicationBackend<App: Application> {
    app: App,
}

impl<App: Application> ApplicationBackend<App> {
    pub fn new() -> Result<Self, JsValue> {
        let backend = Self {
            app: App::default(),
        };

        Ok(backend)
    }

    pub fn run(&self) {
        self.app.run();
    }
}
