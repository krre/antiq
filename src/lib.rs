pub mod core;
pub mod renderer;

pub use core::application::Application;
pub use core::application::ApplicationBackend;
pub use core::context::Context;
pub use core::log::log;
pub use core::window::Window;

pub use renderer::Renderer;

#[macro_export]
macro_rules! run_app {
    ($app_type:ty) => {
        use antiq::ApplicationBackend;
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen(start)]
        pub fn start() -> Result<(), JsValue> {
            let backend = ApplicationBackend::<$app_type>::new()?;
            backend.run();

            Box::leak(Box::new(backend));

            Ok(())
        }
    };
}
