use wasm_bindgen::JsValue;

use crate::{Window, ui::Ui3d};

pub trait Application: Default {
    fn build_ui(&self) -> Ui3d;
}

pub struct ApplicationBackend<App: Application> {
    _app: App,
    _window: Window,
}

impl<App: Application> ApplicationBackend<App> {
    pub async fn new() -> Result<Self, JsValue> {
        let app = App::default();
        let ui = app.build_ui();
        let window = Window::new(ui).await?;

        Ok(Self {
            _app: app,
            _window: window,
        })
    }
}
