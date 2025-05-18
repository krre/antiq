use std::rc::Rc;

use crate::ui::{
    d2::geometry::{Pos2D, Size2D},
    d3::Scene,
    widget::Widget,
};

use super::{Widget2D, Widget2DState};

pub struct View {
    state: Widget2DState,
    scene: Rc<Scene>,
}

impl View {
    pub fn new() -> Self {
        Self {
            state: Widget2DState::new(),
            scene: Rc::new(Scene::new()),
        }
    }

    pub fn set_scene(&mut self, scene: Rc<Scene>) {
        self.scene = scene;
    }

    pub fn scene(&self) -> Rc<Scene> {
        self.scene.clone()
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for View {
    fn build(&self) {}
}

impl Widget2D for View {
    fn set_position(&mut self, position: Pos2D) {
        self.state.position = position;
    }

    fn position(&self) -> Pos2D {
        self.state.position
    }

    fn set_size(&mut self, size: Size2D) {
        self.state.size = size;
    }

    fn size(&self) -> Size2D {
        self.state.size
    }

    fn set_visible(&mut self, visible: bool) {
        self.state.visible = visible;
    }

    fn is_visible(&self) -> bool {
        self.state.visible
    }
}
