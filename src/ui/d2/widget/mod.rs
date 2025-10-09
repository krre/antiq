mod blank;
mod rectangle;
mod view;

pub use blank::Blank;
pub use rectangle::Rectangle;
pub use view::View;

use crate::ui::{
    d2::layout::{Fill2D, Layout2D},
    widget::{HasWidgetState, Widget, WidgetState},
};

use super::geometry::{Pos2D, Size2D};

pub trait HasWidget2DState: HasWidgetState {
    fn widget_state(&self) -> &WidgetState {
        &self.widget_2d_state().state
    }

    fn widget_state_mut(&mut self) -> &mut WidgetState {
        &mut self.widget_2d_state_mut().state
    }

    fn widget_2d_state(&self) -> &Widget2DState;

    fn widget_2d_state_mut(&mut self) -> &mut Widget2DState;
}

pub trait Widget2D: Widget + HasWidget2DState {
    fn set_position(&mut self, position: Pos2D) {
        self.widget_2d_state_mut().position = position;
    }

    fn position(&self) -> Pos2D {
        self.widget_2d_state().position
    }

    fn set_size(&mut self, size: Size2D) {
        self.widget_2d_state_mut().size = size;
    }

    fn size(&self) -> Size2D {
        self.widget_2d_state().size
    }

    fn set_layout(&mut self, layout: Box<dyn Layout2D>) {
        self.widget_2d_state_mut().layout = layout;
    }

    fn layout(&self) -> &Box<dyn Layout2D> {
        &self.widget_2d_state().layout
    }
}

pub struct Widget2DState {
    state: WidgetState,
    position: Pos2D,
    size: Size2D,
    layout: Box<dyn Layout2D>,
}

impl Widget2DState {
    pub fn new() -> Self {
        Self {
            state: WidgetState::new(),
            position: Pos2D::default(),
            size: Size2D::default(),
            layout: Box::new(Fill2D::new()),
        }
    }
}

impl Default for Widget2DState {
    fn default() -> Self {
        Self::new()
    }
}
