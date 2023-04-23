pub trait Widget {
    fn draw(&self);
}

pub struct EmptyWidget {}

impl EmptyWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for EmptyWidget {
    fn draw(&self) {}
}
