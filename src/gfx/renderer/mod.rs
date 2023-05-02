mod scene_renderer;
mod ui_renderer;

pub use scene_renderer::SceneRenderer;
pub use ui_renderer::UIRenderer;

pub trait Renderer {
    fn render(&self);
}
