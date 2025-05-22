use crate::ui::{
    layout::{HasLayoutState, Layout, LayoutState},
    node::{HasNodeState, NodeState},
};

use super::Layout2D;

pub struct Column2D {
    layout_state: LayoutState,
}

impl Column2D {
    pub fn new() -> Self {
        Self {
            layout_state: LayoutState::new(),
        }
    }
}

impl Default for Column2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout for Column2D {
    fn build(&self) {}
}

impl Layout2D for Column2D {}

impl HasNodeState for Column2D {
    fn node_state(&self) -> &NodeState {
        Layout::node_state(self)
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        Layout::node_state_mut(self)
    }
}

impl HasLayoutState for Column2D {
    fn layout_state(&self) -> &LayoutState {
        &self.layout_state
    }

    fn layout_state_mut(&mut self) -> &mut LayoutState {
        &mut self.layout_state
    }
}
