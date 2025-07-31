use wasm_bindgen::JsValue;

use crate::{Renderer, Window, ui::Ui3d};

pub trait Application: Default {
    fn build_ui(&self) -> Ui3d;
}

pub struct ApplicationBackend<App: Application> {
    _app: App,
    window: Window,
    renderer: Renderer,
}

impl<App: Application> ApplicationBackend<App> {
    pub fn new() -> Result<Self, JsValue> {
        let app = App::default();
        let ui = app.build_ui();

        let window = Window::new(ui);
        let renderer = Renderer::new(&window);

        let backend = Self {
            _app: app,
            window,
            renderer,
        };

        Ok(backend)
    }
}
