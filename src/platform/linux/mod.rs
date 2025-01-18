use std::{env, rc::Rc};

use super::{PlatformApplication, PlatformContext, PlatformEventLoop, PlatformWindow};

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
    pub fn new() -> Result<Box<dyn PlatformApplication>, Box<dyn std::error::Error>> {
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
        context: Rc<dyn PlatformContext>,
    ) -> Result<Box<dyn PlatformEventLoop>, Box<dyn std::error::Error>> {
        if backend() == Backend::Wayland {
            wayland::EventLoop::new(context)
        } else {
            x11::EventLoop::new(context)
        }
    }
}

pub struct Window;

impl Window {
    pub fn new(
        context: Rc<dyn PlatformContext>,
    ) -> Result<Box<dyn PlatformWindow>, Box<dyn std::error::Error>> {
        if backend() == Backend::Wayland {
            wayland::Window::new(context)
        } else {
            x11::Window::new(context)
        }
    }
}

pub struct Context;

impl Context {
    pub fn new(
        app: &dyn PlatformApplication,
    ) -> Result<Box<dyn PlatformContext>, Box<dyn std::error::Error>> {
        if backend() == Backend::Wayland {
            wayland::Context::new(app)
        } else {
            x11::Context::new(app)
        }
    }
}
