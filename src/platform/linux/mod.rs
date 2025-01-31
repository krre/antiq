use std::{env, rc::Rc};

use super::{PlatformApplication, PlatformEventLoop, PlatformWindow};

pub mod wayland;
pub mod x11;

#[derive(PartialEq)]
enum Backend {
    Wayland,
    X11,
    Unknown,
}

fn backend() -> Backend {
    if let Ok(session) = env::var("XDG_SESSION_TYPE") {
        if session == "wayland" && env::var("WAYLAND_DISPLAY").is_ok() {
            Backend::Wayland
        } else if session == "x11" && env::var("DISPLAY").is_ok() {
            Backend::X11
        } else {
            Backend::Unknown
        }
    } else {
        Backend::Unknown
    }
}

pub struct Application;

impl Application {
    pub fn new() -> Result<Rc<dyn PlatformApplication>, Box<dyn std::error::Error>> {
        match backend() {
            Backend::Wayland => wayland::Application::new(),
            Backend::X11 => x11::Application::new(),
            Backend::Unknown => panic!("Display server is not supported!"),
        }
    }
}

pub struct EventLoop;

impl EventLoop {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
    ) -> Result<Rc<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        if backend() == Backend::Wayland {
            wayland::EventLoop::new(application)
        } else {
            x11::EventLoop::new(application)
        }
    }
}

pub struct Window;

impl Window {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
        event_loop: Rc<dyn PlatformEventLoop>,
    ) -> Result<Box<dyn PlatformWindow>, Box<dyn std::error::Error>> {
        if backend() == Backend::Wayland {
            wayland::Window::new(application, event_loop)
        } else {
            x11::Window::new(application)
        }
    }
}
