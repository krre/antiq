use std::rc::Rc;

use crate::ui::{
    d3::Scene,
    node::{HasNodeState, Node, NodeState},
    widget::{HasWidgetState, Widget, WidgetState},
};

use super::{HasWidget2DState, Widget2D, Widget2DState};

pub struct View {
    state2d: Widget2DState,
    scene: Rc<Scene>,
}

impl View {
    pub fn new() -> Self {
        Self {
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

impl Node for View {}

impl Widget for View {
    fn build(&self) {}
}

impl Widget2D for View {}

impl HasNodeState for View {
    fn node_state(&self) -> &NodeState {
        Widget::node_state(self)
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        Widget::node_state_mut(self)
    }
}

impl HasWidgetState for View {
    fn widget_state(&self) -> &WidgetState {
        HasWidget2DState::widget_state(self)
    }

    fn widget_state_mut(&mut self) -> &mut WidgetState {
        HasWidget2DState::widget_state_mut(self)
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
