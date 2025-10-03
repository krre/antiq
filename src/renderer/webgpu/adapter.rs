use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::GpuAdapter;

use crate::renderer::webgpu::Device;

pub struct Adapter {
    inner: GpuAdapter,
}

impl Adapter {
    pub(crate) fn new(adapter: GpuAdapter) -> Self {
        Self { inner: adapter }
    }

    pub async fn request_device(&self) -> Result<Device, JsValue> {
        let device = JsFuture::from(self.inner.request_device())
            .await?
            .dyn_into::<web_sys::GpuDevice>()?;
        Ok(Device::new(device))
    }

    pub fn into_inner(&self) -> &GpuAdapter {
        &self.inner
    }
}
