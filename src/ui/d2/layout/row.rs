use crate::ui::{
    layout::{HasLayoutState, Layout, LayoutState},
    node::{HasNodeState, Node, NodeState},
    widget::{HasWidgetState, Widget, WidgetState},
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
}

impl Default for Row2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Row2D {}

impl Widget for Row2D {
    fn build(&self) {}
}

impl Layout for Row2D {
    fn build(&self) {}
}

impl Layout2D for Row2D {}

impl HasNodeState for Row2D {
    fn node_state(&self) -> &NodeState {
        Widget::node_state(self)
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        Widget::node_state_mut(self)
    }
}

impl HasWidgetState for Row2D {
    fn widget_state(&self) -> &WidgetState {
        Layout::widget_state(self)
    }

    fn widget_state_mut(&mut self) -> &mut WidgetState {
        Layout::widget_state_mut(self)
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
