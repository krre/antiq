use crate::ui::widget::Widget;

use super::Layout2D;

pub struct Fill {
    widget: Box<dyn Widget>,
}

impl Fill {
    pub fn new() -> Self {
        Self {
            widget: Box::new(()),
        }
    }

    pub fn set_widget(&mut self, widget: Box<dyn Widget>) {
        self.widget = widget;
    }
}

impl Layout2D for Fill {
    fn build(&self) {}
}
