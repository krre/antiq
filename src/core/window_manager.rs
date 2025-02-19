use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    window::{Window, WindowId},
    Pos2D, Size2D,
};

pub(crate) struct WindowManager {
    windows: RefCell<HashMap<WindowId, Rc<Window>>>,
}

impl WindowManager {
    pub(crate) fn new() -> Self {
        Self {
            windows: RefCell::new(HashMap::new()),
        }
    }

    pub(crate) fn append(&self, id: WindowId, window: Rc<Window>) {
        #[allow(unused_variables)]
        let window = self.windows.borrow_mut().insert(id, window);
    }

    pub(crate) fn remove(&self, id: WindowId) {
        #[allow(unused_variables)]
        let window = self.windows.borrow_mut().remove(&id);
    }

    pub(crate) fn render(&self, id: WindowId) {
        self.windows.borrow().get(&id).unwrap().render();
    }

    pub(crate) fn resize(&self, id: WindowId, size: Size2D) {
        self.windows.borrow().get(&id).unwrap().update_size(size);
    }

    pub(crate) fn move_to(&self, id: WindowId, pos: Pos2D) {
        self.windows.borrow().get(&id).unwrap().update_position(pos);
    }

    pub(crate) fn count(&self) -> usize {
        self.windows.borrow().len()
    }
}
