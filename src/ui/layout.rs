use std::{cell::RefCell, rc::Rc};

use crate::ui::widget::{HasWidgetState, WidgetState};

use super::widget::Widget;

pub trait HasLayoutState {
    fn layout_state(&self) -> &LayoutState;

    fn layout_state_mut(&mut self) -> &mut LayoutState;
}

pub trait Layout: Widget + HasLayoutState + HasWidgetState {
    fn add_widget(&mut self, widget: Rc<RefCell<dyn Widget>>) {
        self.add_child(widget);
    }

    fn build(&self);

    fn widget_state(&self) -> &WidgetState {
        &self.layout_state().widget_state
    }

    fn widget_state_mut(&mut self) -> &mut WidgetState {
        &mut self.layout_state_mut().widget_state
    }
}

pub struct LayoutState {
    widget_state: WidgetState,
}

impl LayoutState {
    pub fn new() -> Self {
        Self {
            widget_state: WidgetState::new(),
        }
    }
}

impl Default for LayoutState {
    fn default() -> Self {
        Self::new()
    }
}
