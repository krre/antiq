use super::Window;

pub struct Application {}

impl Application {
    pub fn run() {
        println!("Application started");
    }

    pub fn create_window() -> Window {
        Window::new()
    }
}
