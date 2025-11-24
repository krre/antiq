use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::GpuAdapter;

pub struct Gpu {
    inner: web_sys::Gpu,
}

impl Gpu {
    pub(crate) fn new(gpu: web_sys::Gpu) -> Self {
        Self { inner: gpu }
    }

    pub async fn request_adapter(&self) -> Result<GpuAdapter, JsValue> {
        let adapter = JsFuture::from(self.inner.request_adapter())
            .await?
            .dyn_into::<web_sys::GpuAdapter>()?;
        Ok(adapter)
    }

    pub fn into_inner(&self) -> &web_sys::Gpu {
        &self.inner
    }
}
