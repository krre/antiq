use super::PlatformWindow;

#[derive(Default, Debug)]
pub struct Window {}

impl PlatformWindow for Window {
    fn set_title(&mut self, title: &str) {
        dbg!(title);
    }
}
