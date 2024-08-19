use super::Renderer;

pub struct SceneRenderer {}

impl SceneRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Renderer for SceneRenderer {
    fn render(&self) {}
}
