pub struct GpuDevice {
    _inner: web_sys::GpuDevice,
}

impl GpuDevice {
    pub(crate) fn new(gpu_device: web_sys::GpuDevice) -> Self {
        Self { _inner: gpu_device }
    }
}
