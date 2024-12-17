use std::any::Any;

use crate::core::Pos2D;

cfg_if::cfg_if! {
    if #[cfg(linux)] {
        pub mod linux;
        #[allow(unused_imports)]
        pub use linux::*;
    } else if #[cfg(macos)] {
        pub mod macos;
        #[allow(unused_imports)]
        pub use macos::*;
    } else if #[cfg(win64)] {
        pub mod windows;
        #[allow(unused_imports)]
        pub use windows::*;
    } else {
        compile_error!("Platform not supported")
    }
}

pub trait PlatformApplication: Any {
    fn as_any(&self) -> &dyn Any;
}

pub trait PlatformWindow: Any {
    fn as_any(&self) -> &dyn Any;

    fn set_title(&self, title: &str);

    fn title(&self) -> String;

    fn set_visible(&self, visible: bool);

    fn set_position(&self, pos: Pos2D);
}

pub trait PlatformEventLoop: Any {
    fn as_any(&self) -> &dyn Any;

    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait PlatformContext: Any {
    fn as_any(&self) -> &dyn Any;
}
