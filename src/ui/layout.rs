use std::{cell::RefCell, rc::Rc};

use super::{
    node::{HasNodeState, Node, NodeState},
    widget::Widget,
};

pub trait HasLayoutState {
    fn layout_state(&self) -> &LayoutState;

    fn layout_state_mut(&mut self) -> &mut LayoutState;
}

pub trait Layout: Node + HasLayoutState + HasNodeState {
    fn add_widget(&mut self, widget: Rc<RefCell<dyn Widget>>) {
        self.add_child(widget);
    }

    fn build(&self);

    fn node_state(&self) -> &NodeState {
        &self.layout_state().node_state
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        &mut self.layout_state_mut().node_state
    }
}

pub struct LayoutState {
    node_state: NodeState,
}

impl LayoutState {
    pub fn new() -> Self {
        Self {
            node_state: NodeState::new(),
        }
    }
}

impl Default for LayoutState {
    fn default() -> Self {
        Self::new()
    }
}
