use std::{env, rc::Rc};

use crate::{core::Result, ui::d2::geometry::Size2D};

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
    pub fn new() -> Result<Rc<dyn PlatformApplication>> {
        match backend() {
            Backend::Wayland => wayland::Application::new(),
            Backend::X11 => x11::Application::new(),
            Backend::Unknown => panic!("Unknown display server!"),
        }
    }
}

pub struct EventLoop;

impl EventLoop {
    pub fn new(application: Rc<dyn PlatformApplication>) -> Result<Rc<dyn PlatformEventLoop>> {
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
        size: Size2D,
    ) -> Result<Box<dyn PlatformWindow>> {
        if backend() == Backend::Wayland {
            wayland::Window::new(application, event_loop, size)
        } else {
            x11::Window::new(application, size)
        }
    }
}
