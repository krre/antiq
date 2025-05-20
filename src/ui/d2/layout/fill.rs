use crate::ui::{d2::widget::Blank, layout::Layout, widget::Widget};

use super::Layout2D;

pub struct Fill2D {
    widget: Box<dyn Widget>,
}

impl Fill2D {
    pub fn new() -> Self {
        Self {
            widget: Box::new(Blank::new()),
        }
    }

    pub fn set_widget(&mut self, widget: Box<dyn Widget>) {
        self.widget = widget;
    }
}

impl Default for Fill2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout for Fill2D {
    fn build(&self) {}
}

impl Layout2D for Fill2D {}
