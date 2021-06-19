use super::Widget;
use crate::core::Id;
use crate::entity::Entity;

pub struct Window {
    id: Id,
}

impl Window {
    pub fn new() -> Self {
        Self { id: Id::default() }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity for Window {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Window {
    fn draw(&self) {}
}
