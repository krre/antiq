use crate::ui::{
    Color,
    d2::geometry::{Pos2D, Size2D},
    widget::Widget,
};

use super::{Widget2D, Widget2DState};

pub struct Rectangle {
    state: Widget2DState,
    color: Color,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            state: Widget2DState::new(),
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

impl Widget for Rectangle {
    fn build(&self) {}
}

impl Widget2D for Rectangle {
    fn set_position(&mut self, position: Pos2D) {
        self.state.position = position;
    }

    fn position(&self) -> Pos2D {
        self.state.position
    }

    fn set_size(&mut self, size: Size2D) {
        self.state.size = size;
    }

    fn size(&self) -> Size2D {
        self.state.size
    }

    fn set_visible(&mut self, visible: bool) {
        self.state.visible = visible;
    }

    fn is_visible(&self) -> bool {
        self.state.visible
    }
}
