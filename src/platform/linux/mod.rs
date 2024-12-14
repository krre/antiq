use std::env;

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
    if let Ok(_) = env::var("WAYLAND_DISPLAY") {
        Backend::Wayland
    } else if let Ok(_) = env::var("DISPLAY") {
        Backend::X11
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

pub struct Context;

impl Context {
    pub fn new() -> Result<Box<dyn PlatformContext>, Box<dyn std::error::Error>> {
        if backend() == Backend::Wayland {
            wayland::Context::new()
        } else {
            x11::Context::new()
        }
    }
}
