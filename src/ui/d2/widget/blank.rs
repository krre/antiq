use crate::ui::widget::{HasWidgetState, Widget, WidgetState};

use super::{HasWidget2DState, Widget2DState};

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

impl HasWidget2DState for Blank {
    fn widget_2d_state(&self) -> &Widget2DState {
        &self.state2d
    }

    fn widget_2d_state_mut(&mut self) -> &mut Widget2DState {
        &mut self.state2d
    }
}
