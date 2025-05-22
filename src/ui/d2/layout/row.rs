use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::ui::{
    d2::widget::Widget2D,
    layout::{HasLayoutState, Layout, LayoutState},
    node::{HasNodeState, NodeState},
};

use super::Layout2D;

pub struct Row2D {
    layout_state: LayoutState,
    widgets: Vec<Rc<RefCell<dyn Widget2D>>>,
}

impl Row2D {
    pub fn new() -> Self {
        Self {
            layout_state: LayoutState::new(),
            widgets: Vec::new(),
        }
    }

    pub fn add_widget<T: Widget2D + 'static>(&mut self, widget: T) -> Weak<RefCell<T>> {
        let rc = Rc::new(RefCell::new(widget));
        let weak = Rc::downgrade(&rc);
        self.widgets.push(rc);
        weak
    }
}

impl Default for Row2D {
    fn default() -> Self {
        Self::new()
    }
}

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
