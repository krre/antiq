use crate::ui::{
    layout::{HasLayoutState, Layout, LayoutState},
    node::{HasNodeState, Node, NodeState},
};

use super::Layout2D;

pub struct Split2D {
    layout_state: LayoutState,
}

impl Split2D {
    pub fn new() -> Self {
        Self {
            layout_state: LayoutState::new(),
        }
    }
}

impl Default for Split2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Split2D {}

impl Layout for Split2D {
    fn build(&self) {}
}

impl Layout2D for Split2D {}

impl HasNodeState for Split2D {
    fn node_state(&self) -> &NodeState {
        Layout::node_state(self)
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        Layout::node_state_mut(self)
    }
}

impl HasLayoutState for Split2D {
    fn layout_state(&self) -> &LayoutState {
        &self.layout_state
    }

    fn layout_state_mut(&mut self) -> &mut LayoutState {
        &mut self.layout_state
    }
}
