use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    Pos2D, Size2D,
    window::{Window, WindowId},
};

pub(crate) struct WindowManager {
    windows: HashMap<WindowId, Rc<RefCell<Window>>>,
}

impl WindowManager {
    pub(crate) fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub(crate) fn append(&mut self, id: WindowId, window: Rc<RefCell<Window>>) {
        #[allow(unused_variables)]
        let window = self.windows.insert(id, window);
    }

    pub(crate) fn remove(&mut self, id: WindowId) {
        #[allow(unused_variables)]
        let window = self.windows.remove(&id);
    }

    pub(crate) fn render(&self, id: WindowId) {
        self.windows.get(&id).map(|w| w.borrow().render());
    }

    pub(crate) fn resize(&self, id: WindowId, size: Size2D) {
        self.windows
            .get(&id)
            .map(|w| w.borrow_mut().update_size(size));
    }

    pub(crate) fn ask_resize(&self, id: WindowId, size: Size2D) {
        self.windows.get(&id).map(|w| w.borrow_mut().set_size(size));
    }

    pub(crate) fn move_to(&self, id: WindowId, pos: Pos2D) {
        self.windows
            .get(&id)
            .map(|w| w.borrow_mut().update_position(pos));
    }

    pub(crate) fn count(&self) -> usize {
        self.windows.len()
    }
}
