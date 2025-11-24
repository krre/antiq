use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{GpuAdapter, GpuDevice};

pub struct Adapter {
    inner: GpuAdapter,
}

impl Adapter {
    pub(crate) fn new(adapter: GpuAdapter) -> Self {
        Self { inner: adapter }
    }

    pub async fn request_device(&self) -> Result<GpuDevice, JsValue> {
        let device = JsFuture::from(self.inner.request_device())
            .await?
            .dyn_into::<web_sys::GpuDevice>()?;
        Ok(device)
    }

    pub fn into_inner(&self) -> &GpuAdapter {
        &self.inner
    }
}
