pub struct Device {
    _inner: web_sys::GpuDevice,
}

impl Device {
    pub(crate) fn new(device: web_sys::GpuDevice) -> Self {
        Self { _inner: device }
    }
}
