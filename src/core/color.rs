#[derive(Debug)]
pub struct Color(wgpu::Color);

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self {
            0: wgpu::Color { r, g, b, a },
        }
    }

    pub(crate) fn inner(&self) -> wgpu::Color {
        self.0
    }
}
