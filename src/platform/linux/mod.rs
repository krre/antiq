use super::{PlatformApplication, PlatformEventLoop, PlatformWindow};

pub mod wayland;
pub mod x11;

#[derive(PartialEq)]
enum Backend {
    Wayland,
    X11,
}

fn backend() -> Backend {
    Backend::X11
}

pub struct Application;

impl Application {
    pub fn new() -> Result<Box<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        if backend() == Backend::Wayland {
            wayland::Application::new()
        } else {
            x11::Application::new()
        }
    }
}

pub struct EventLoop;

impl EventLoop {
    pub fn new() -> Result<Box<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        if backend() == Backend::Wayland {
            wayland::EventLoop::new()
        } else {
            x11::EventLoop::new()
        }
    }
}

pub struct Window;

impl Window {
    pub fn new() -> Result<Box<dyn PlatformWindow>, Box<dyn std::error::Error>> {
        if backend() == Backend::Wayland {
            wayland::Window::new()
        } else {
            x11::Window::new()
        }
    }
}
