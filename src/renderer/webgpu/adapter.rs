use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::renderer::webgpu::Device;

pub struct Adapter {
    _inner: web_sys::GpuAdapter,
}

impl Adapter {
    pub(crate) fn new(adapter: web_sys::GpuAdapter) -> Self {
        Self { _inner: adapter }
    }

    pub async fn request_device(&self) -> Result<Device, JsValue> {
        let device = JsFuture::from(self._inner.request_device())
            .await?
            .dyn_into::<web_sys::GpuDevice>()?;
        Ok(Device::new(device))
    }
}
