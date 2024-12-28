use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{Pos2D, Size2D, Window, WindowId};

pub(crate) struct WindowManager {
    windows: RefCell<HashMap<WindowId, Rc<Window>>>,
}

impl WindowManager {
    pub(crate) fn new() -> Self {
        Self {
            windows: RefCell::new(HashMap::new()),
        }
    }

    pub(crate) fn add_window(&self, id: WindowId, window: Rc<Window>) {
        self.windows.borrow_mut().insert(id, window);
    }

    pub(crate) fn remove_window(&self, id: WindowId) {
        #[allow(unused_variables)]
        let window = self.windows.borrow_mut().remove(&id);
    }

    pub(crate) fn render_window(&self, id: WindowId) {
        self.windows.borrow().get(&id).unwrap().render();
    }

    pub(crate) fn update_window_size(&self, id: WindowId, size: Size2D) {
        self.windows.borrow().get(&id).unwrap().update_size(size);
    }

    pub(crate) fn update_window_position(&self, id: WindowId, pos: Pos2D) {
        self.windows.borrow().get(&id).unwrap().update_position(pos);
    }
}
