use crate::window::Windowed;

#[derive(Default)]
pub struct Window {}

impl Windowed for Window {
    fn set_title(&mut self, _title: &str) {}

    fn title(&self) -> &str {
        "Linux window"
    }
}
