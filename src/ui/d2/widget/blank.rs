use crate::ui::{
    d2::geometry::{Pos2D, Size2D},
    widget::{HasWidgetState, Widget, WidgetState},
};

use super::{Widget2D, Widget2DState};

pub struct Blank {
    state: WidgetState,
    state2d: Widget2DState,
}

impl Blank {
    pub fn new() -> Self {
        Self {
            state: WidgetState::new(),
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

impl HasWidgetState for Blank {
    fn widget_state(&self) -> &WidgetState {
        &self.state
    }

    fn widget_state_mut(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}

impl Widget2D for Blank {
    fn set_position(&mut self, position: Pos2D) {
        self.state2d.position = position;
    }

    fn position(&self) -> Pos2D {
        self.state2d.position
    }

    fn set_size(&mut self, size: Size2D) {
        self.state2d.size = size;
    }

    fn size(&self) -> Size2D {
        self.state2d.size
    }
}
