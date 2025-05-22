use crate::ui::{
    d2::widget::Blank,
    layout::{HasLayoutState, Layout, LayoutState},
    node::{HasNodeState, NodeState},
    widget::Widget,
};

use super::Layout2D;

pub struct Fill2D {
    layout_state: LayoutState,
    widget: Box<dyn Widget>,
}

impl Fill2D {
    pub fn new() -> Self {
        Self {
            layout_state: LayoutState::new(),
            widget: Box::new(Blank::new()),
        }
    }

    pub fn set_widget(&mut self, widget: Box<dyn Widget>) {
        self.widget = widget;
    }
}

impl Default for Fill2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout for Fill2D {
    fn build(&self) {}
}

impl Layout2D for Fill2D {}

impl HasNodeState for Fill2D {
    fn node_state(&self) -> &NodeState {
        Layout::node_state(self)
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        Layout::node_state_mut(self)
    }
}

impl HasLayoutState for Fill2D {
    fn layout_state(&self) -> &LayoutState {
        &self.layout_state
    }

    fn layout_state_mut(&mut self) -> &mut LayoutState {
        &mut self.layout_state
    }
}
