use super::Widget;
use super::Window;
use crate::core::Id;
use crate::entity::Entity;

pub struct ApplicationWindow {
    window: Window,
}

impl Entity for ApplicationWindow {
    fn id(&self) -> Id {
        self.window.id()
    }
}

impl Widget for ApplicationWindow {
    fn draw(&self) {
        self.window.draw();
    }
}
