use crate::ui::{
    Color,
    d2::geometry::{Pos2D, Size2D},
    widget::{HasWidgetState, Widget, WidgetState},
};

use super::{Widget2D, Widget2DState};

pub struct Rectangle {
    state: WidgetState,
    state2d: Widget2DState,
    color: Color,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            state: WidgetState::new(),
            state2d: Widget2DState::new(),
            color: Color::new(1.0, 1.0, 1.0),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Rectangle {
    fn build(&self) {}
}

impl HasWidgetState for Rectangle {
    fn widget_state(&self) -> &WidgetState {
        &self.state
    }

    fn widget_state_mut(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}

impl Widget2D for Rectangle {
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
