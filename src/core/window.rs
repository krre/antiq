use crate::ui::d2::geometry::Size2D;

pub struct Window {
    inner: web_sys::Window,
}

impl Window {
    pub fn new() -> Self {
        Self {
            inner: web_sys::window().unwrap(),
        }
    }

    pub fn size(&self) -> Size2D {
        let width = self.inner.inner_width().ok().unwrap().as_f64().unwrap() as u32;
        let height = self.inner.inner_height().ok().unwrap().as_f64().unwrap() as u32;
        Size2D::new(width, height)
    }
}
