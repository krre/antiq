use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::ui::{d2::widget::Widget2D, layout::Layout};

use super::Layout2D;

pub struct Row2D {
    widgets: Vec<Rc<RefCell<dyn Widget2D>>>,
}

impl Row2D {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add_widget<T: Widget2D + 'static>(&mut self, widget: T) -> Weak<RefCell<T>> {
        let rc = Rc::new(RefCell::new(widget));
        let weak = Rc::downgrade(&rc);
        self.widgets.push(rc);
        weak
    }
}

impl Layout for Row2D {
    fn build(&self) {}
}

impl Layout2D for Row2D {}
