use super::Renderer;

pub struct UIRenderer {}

impl UIRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Renderer for UIRenderer {
    fn render(&self) {}
}
