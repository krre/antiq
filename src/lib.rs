pub mod core;
pub use core::application::Application;

#[macro_export]
macro_rules! run_app {
    ($app_type:ty) => {
        #[wasm_bindgen::prelude::wasm_bindgen(start)]
        pub fn start() {
            let app: $app_type = Default::default();
            app.run();
        }
    };
}
