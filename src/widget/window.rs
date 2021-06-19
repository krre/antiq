use super::Widget;
use crate::core::Id;
use crate::entity::Entity;

#[derive(Default)]
pub struct Window {
    id: Id,
}

impl Window {
    pub fn new() -> Self {
        Self { id: Id::default() }
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
