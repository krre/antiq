pub struct GpuAdapter {
    _inner: web_sys::GpuAdapter,
}

impl GpuAdapter {
    pub(crate) fn new(gpu_adapter: web_sys::GpuAdapter) -> Self {
        Self {
            _inner: gpu_adapter,
        }
    }
}
