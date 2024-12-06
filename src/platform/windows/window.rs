use crate::platform::PlatformWindow;

pub struct Window {}

impl Window {
    pub fn new() -> Self {
        Self {}
    }
}

impl PlatformWindow for Window {}
