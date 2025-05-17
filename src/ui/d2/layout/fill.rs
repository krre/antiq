use crate::ui::{layout::Layout, widget::Widget};

use super::Layout2D;

pub struct Fill2D {
    widget: Box<dyn Widget>,
}

impl Fill2D {
    pub fn new() -> Self {
        Self {
            widget: Box::new(()),
        }
    }

    pub fn set_widget(&mut self, widget: Box<dyn Widget>) {
        self.widget = widget;
    }
}

impl Layout for Fill2D {
    fn build(&self) {}
}

impl Layout2D for Fill2D {}
