use std::rc::Rc;

use crate::ui::{d3::Scene, widget::Widget};

use super::Widget2D;

pub struct View {
    scene: Rc<Scene>,
}

impl View {
    pub fn new() -> Self {
        Self {
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

impl Widget for View {
    fn build(&self) {}
}

impl Widget2D for View {}
