use web_sys::GpuDevice;

pub struct Device {
    inner: GpuDevice,
}

impl Device {
    pub(crate) fn new(device: GpuDevice) -> Self {
        Self { inner: device }
    }

    pub fn into_inner(&self) -> &GpuDevice {
        &self.inner
    }
}
