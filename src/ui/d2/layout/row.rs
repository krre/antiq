use std::{cell::RefCell, rc::Rc};

use crate::ui::{
    d2::widget::Widget2D,
    layout::{HasLayoutState, Layout, LayoutState},
    node::{HasNodeState, Node, NodeState},
};

use super::Layout2D;

pub struct Row2D {
    layout_state: LayoutState,
}

impl Row2D {
    pub fn new() -> Self {
        Self {
            layout_state: LayoutState::new(),
        }
    }

    pub fn add_widget2d<T: Widget2D + 'static>(&mut self, widget: T) {
        self.add_widget(Rc::new(RefCell::new(widget)));
    }
}

impl Default for Row2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Row2D {}

impl Layout for Row2D {
    fn build(&self) {}
}

impl Layout2D for Row2D {}

impl HasNodeState for Row2D {
    fn node_state(&self) -> &NodeState {
        Layout::node_state(self)
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        Layout::node_state_mut(self)
    }
}

impl HasLayoutState for Row2D {
    fn layout_state(&self) -> &LayoutState {
        &self.layout_state
    }

    fn layout_state_mut(&mut self) -> &mut LayoutState {
        &mut self.layout_state
    }
}
