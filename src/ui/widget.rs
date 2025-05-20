pub trait Widget {
    fn set_visible(&mut self, visible: bool);

    fn is_visible(&self) -> bool;

    fn set_opactity(&mut self, opacity: f32);

    fn opacity(&self) -> f32;

    fn build(&self);
}

pub struct WidgetState {
    pub(crate) visible: bool,
    pub(crate) opacity: f32,
}

impl WidgetState {
    pub fn new() -> Self {
        Self {
            visible: true,
            opacity: 1.0,
        }
    }
}

impl Default for WidgetState {
    fn default() -> Self {
        Self::new()
    }
}
