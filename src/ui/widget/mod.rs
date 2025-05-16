pub trait Widget {
    fn build(&self);
}

impl Widget for () {
    fn build(&self) {}
}
