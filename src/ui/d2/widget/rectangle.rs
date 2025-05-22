use crate::ui::{
    Color,
    widget::{HasWidgetState, Widget, WidgetState},
};

use super::{HasWidget2DState, Widget2D, Widget2DState};

pub struct Rectangle {
    state2d: Widget2DState,
    color: Color,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
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

impl Widget2D for Rectangle {}

impl HasWidgetState for Rectangle {
    fn widget_state(&self) -> &WidgetState {
        HasWidget2DState::widget_state(self)
    }

    fn widget_state_mut(&mut self) -> &mut WidgetState {
        HasWidget2DState::widget_state_mut(self)
    }
}

impl HasWidget2DState for Rectangle {
    fn widget_2d_state(&self) -> &Widget2DState {
        &self.state2d
    }

    fn widget_2d_state_mut(&mut self) -> &mut Widget2DState {
        &mut self.state2d
    }
}
