use crate::ui::{
    layout::{HasLayoutState, Layout, LayoutState},
    node::{HasNodeState, Node, NodeState},
};

use super::Layout2D;

pub struct Position2D {
    layout_state: LayoutState,
}

impl Position2D {
    pub fn new() -> Self {
        Self {
            layout_state: LayoutState::new(),
        }
    }
}

impl Default for Position2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Position2D {}

impl Layout for Position2D {
    fn build(&self) {}
}

impl Layout2D for Position2D {}

impl HasNodeState for Position2D {
    fn node_state(&self) -> &NodeState {
        Layout::node_state(self)
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        Layout::node_state_mut(self)
    }
}

impl HasLayoutState for Position2D {
    fn layout_state(&self) -> &LayoutState {
        &self.layout_state
    }

    fn layout_state_mut(&mut self) -> &mut LayoutState {
        &mut self.layout_state
    }
}
