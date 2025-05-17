mod rectangle;
mod view;

pub use rectangle::Rectangle;
pub use view::View;

use super::geometry::{Pos2D, Size2D};

pub trait Widget2D {
    fn set_position(&mut self, position: Pos2D);
    fn position(&self) -> Pos2D;

    fn set_size(&mut self, size: Size2D);
    fn size(&self) -> Size2D;

    fn set_visible(&mut self, visible: bool);
    fn is_visible(&self) -> bool;
}

pub struct Widget2DState {
    position: Pos2D,
    size: Size2D,
    visible: bool,
}

impl Widget2DState {
    pub fn new() -> Self {
        Self {
            position: Pos2D::default(),
            size: Size2D::default(),
            visible: true,
        }
    }
}
