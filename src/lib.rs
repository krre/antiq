pub mod core;

pub use core::application::Application;
pub use core::log::log;

pub use core::application::ApplicationBackend;

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
