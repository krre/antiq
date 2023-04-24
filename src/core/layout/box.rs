use crate::widget::{EmptyWidget, Widget};

use super::Layout;

pub struct Box {
    widget: std::boxed::Box<dyn Widget>,
}

impl Box {
    pub fn new() -> Self {
        Self {
            widget: std::boxed::Box::new(EmptyWidget::new()),
        }
    }

    pub fn set_widget(&mut self, widget: std::boxed::Box<dyn Widget>) {
        self.widget = widget;
    }
}

impl Layout for Box {
    fn draw(&self) {
        log::info!("Draw box layout");
    }
}
