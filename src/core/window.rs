use crate::ui::d2::geometry::Size2D;

pub struct Window(web_sys::Window);

impl Window {
    pub fn new() -> Self {
        Self {
            0: web_sys::window().unwrap(),
        }
    }

    pub fn size(&self) -> Size2D {
        let width = self.0.inner_width().ok().unwrap().as_f64().unwrap() as u32;
        let height = self.0.inner_height().ok().unwrap().as_f64().unwrap() as u32;
        Size2D::new(width, height)
    }
}
