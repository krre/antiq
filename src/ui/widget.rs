use super::node::{HasNodeState, Node, NodeState};

pub trait HasWidgetState {
    fn widget_state(&self) -> &WidgetState;

    fn widget_state_mut(&mut self) -> &mut WidgetState;
}

pub trait Widget: Node + HasWidgetState + HasNodeState {
    fn set_visible(&mut self, visible: bool) {
        self.widget_state_mut().visible = visible;
    }

    fn is_visible(&self) -> bool {
        self.widget_state().visible
    }

    fn set_opactity(&mut self, opacity: f32) {
        self.widget_state_mut().opacity = opacity;
    }

    fn opacity(&self) -> f32 {
        self.widget_state().opacity
    }

    fn build(&self);

    fn node_state(&self) -> &NodeState {
        &self.widget_state().node_state
    }

    fn node_state_mut(&mut self) -> &mut NodeState {
        &mut self.widget_state_mut().node_state
    }
}

pub struct WidgetState {
    node_state: NodeState,
    pub(crate) visible: bool,
    pub(crate) opacity: f32,
}

impl WidgetState {
    pub fn new() -> Self {
        Self {
            node_state: NodeState::new(),
            visible: true,
            opacity: 1.0,
        }
    }
}

impl Default for WidgetState {
    fn default() -> Self {
        Self::new()
    }
}
