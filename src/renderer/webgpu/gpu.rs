pub struct Gpu {
    _inner: web_sys::Gpu,
}

impl Gpu {
    pub(crate) fn new(gui: web_sys::Gpu) -> Self {
        Self { _inner: gui }
    }
}
