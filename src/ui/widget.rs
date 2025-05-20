pub trait Widget {
    fn set_visible(&mut self, visible: bool);

    fn is_visible(&self) -> bool;

    fn build(&self);
}

impl Widget for () {
    fn set_visible(&mut self, visible: bool) {
        let _ = visible;
    }

    fn is_visible(&self) -> bool {
        false
    }

    fn build(&self) {}
}

pub struct WidgetState {
    pub(crate) visible: bool,
}

impl WidgetState {
    pub fn new() -> Self {
        Self { visible: true }
    }
}

impl Default for WidgetState {
    fn default() -> Self {
        Self::new()
    }
}
