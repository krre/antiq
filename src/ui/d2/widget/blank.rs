use crate::ui::{
    node::{HasNodeState, NodeState},
    widget::{HasWidgetState, Widget, WidgetState},
};

use super::{HasWidget2DState, Widget2D, Widget2DState};

pub struct Blank {
    state2d: Widget2DState,
}

impl Blank {
    pub fn new() -> Self {
        Self {
            state2d: Widget2DState::new(),
        }
    }
}

impl Default for Blank {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Blank {
    fn build(&self) {}
}

impl Widget2D for Blank {}

impl HasNodeState for Blank {
    fn node_state(&self) -> &NodeState {
        Widget::node_state(self)
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        Widget::node_state_mut(self)
    }
}

impl HasWidgetState for Blank {
    fn widget_state(&self) -> &WidgetState {
        HasWidget2DState::widget_state(self)
    }

    fn widget_state_mut(&mut self) -> &mut WidgetState {
        HasWidget2DState::widget_state_mut(self)
    }
}

impl HasWidget2DState for Blank {
    fn widget_2d_state(&self) -> &Widget2DState {
        &self.state2d
    }

    fn widget_2d_state_mut(&mut self) -> &mut Widget2DState {
        &mut self.state2d
    }
}
