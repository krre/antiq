use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use super::Adapter;

pub struct Gpu {
    _inner: web_sys::Gpu,
}

impl Gpu {
    pub(crate) fn new(gui: web_sys::Gpu) -> Self {
        Self { _inner: gui }
    }

    pub async fn request_adapter(&self) -> Result<Adapter, JsValue> {
        let adapter = JsFuture::from(self._inner.request_adapter())
            .await?
            .dyn_into::<web_sys::GpuAdapter>()?;
        Ok(Adapter::new(adapter))
    }
}
