use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::renderer::webgpu::GpuDevice;

pub struct GpuAdapter {
    _inner: web_sys::GpuAdapter,
}

impl GpuAdapter {
    pub(crate) fn new(gpu_adapter: web_sys::GpuAdapter) -> Self {
        Self {
            _inner: gpu_adapter,
        }
    }

    pub async fn request_device(&self) -> Result<GpuDevice, JsValue> {
        let device = JsFuture::from(self._inner.request_device())
            .await?
            .dyn_into::<web_sys::GpuDevice>()?;
        Ok(GpuDevice::new(device))
    }
}
