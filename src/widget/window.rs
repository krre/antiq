use super::Widget;
use crate::core::Id;
use crate::entity::Entity;

pub struct Window {
    id: Id,
}

impl Entity for Window {
    fn id(&self) -> Id {
        self.id
    }
}

impl Widget for Window {
    fn draw(&self) {}
}
