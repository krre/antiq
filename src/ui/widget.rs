pub trait HasWidgetState {
    fn widget_state(&self) -> &WidgetState;

    fn widget_state_mut(&mut self) -> &mut WidgetState;
}

pub trait Widget: HasWidgetState {
    fn set_visible(&mut self, visible: bool) {
        self.widget_state_mut().visible = visible;
    }

    fn is_visible(&self) -> bool {
        self.widget_state().visible
    }

    fn set_opactity(&mut self, opacity: f32) {
        self.widget_state_mut().opacity = opacity;
    }

    fn opacity(&self) -> f32 {
        self.widget_state().opacity
    }

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
