use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ui::d2::geometry::{Pos2D, Size2D};

use super::window::{Window, WindowId};

pub(crate) struct WindowManager {
    windows: HashMap<WindowId, Rc<RefCell<Window>>>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn append(&mut self, id: WindowId, window: Rc<RefCell<Window>>) {
        #[allow(unused_variables)]
        let window = self.windows.insert(id, window);
    }

    pub fn remove(&mut self, id: WindowId) {
        #[allow(unused_variables)]
        let window = self.windows.remove(&id);
    }

    pub fn render(&self, id: WindowId) {
        if let Some(w) = self.windows.get(&id) {
            w.borrow().render()
        }
    }

    pub fn resize(&self, id: WindowId, size: Size2D) {
        if let Some(w) = self.windows.get(&id) {
            w.borrow_mut().update_size(size)
        }
    }

    pub fn ask_resize(&self, id: WindowId, size: Size2D) {
        if let Some(w) = self.windows.get(&id) {
            w.borrow_mut().set_size(size)
        }
    }

    pub fn move_to(&self, id: WindowId, pos: Pos2D) {
        if let Some(w) = self.windows.get(&id) {
            w.borrow_mut().update_position(pos)
        }
    }

    pub fn count(&self) -> usize {
        self.windows.len()
    }
}
