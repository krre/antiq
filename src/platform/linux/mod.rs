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
        let app: Rc<dyn PlatformApplication> = match backend() {
            Backend::Wayland => Rc::new(wayland::Application::new()?),
            Backend::X11 => Rc::new(x11::Application::new()?),
            Backend::Unknown => panic!("Unknown display server!"),
        };

        Ok(app)
    }
}

pub struct EventLoop;

impl EventLoop {
    pub fn new(application: Rc<dyn PlatformApplication>) -> Result<Rc<dyn PlatformEventLoop>> {
        let event_loop: Rc<dyn PlatformEventLoop> = if backend() == Backend::Wayland {
            Rc::new(wayland::EventLoop::new(application)?)
        } else {
            Rc::new(x11::EventLoop::new(application)?)
        };

        Ok(event_loop)
    }
}

pub struct Window;

impl Window {
    pub fn new(
        application: Rc<dyn PlatformApplication>,
        event_loop: Rc<dyn PlatformEventLoop>,
        size: Size2D,
    ) -> Result<Rc<dyn PlatformWindow>> {
        let window: Rc<dyn PlatformWindow> = if backend() == Backend::Wayland {
            Rc::new(wayland::Window::new(application, event_loop, size)?)
        } else {
            Rc::new(x11::Window::new(application, size)?)
        };

        Ok(window)
    }
}
