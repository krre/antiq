use std::rc::Rc;

use crate::ui::{
    d3::Scene,
    widget::{HasWidgetState, Widget, WidgetState},
};

use super::{HasWidget2DState, Widget2DState};

pub struct View {
    state: WidgetState,
    state2d: Widget2DState,
    scene: Rc<Scene>,
}

impl View {
    pub fn new() -> Self {
        Self {
            state: WidgetState::new(),
            state2d: Widget2DState::new(),
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

impl HasWidgetState for View {
    fn widget_state(&self) -> &WidgetState {
        &self.state
    }

    fn widget_state_mut(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}

impl HasWidget2DState for View {
    fn widget_2d_state(&self) -> &Widget2DState {
        &self.state2d
    }

    fn widget_2d_state_mut(&mut self) -> &mut Widget2DState {
        &mut self.state2d
    }
}
