use std::any::Any;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::SurfaceTarget;

use crate::core::{Pos2D, Size2D};

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

    fn set_size(&self, size: Size2D);
}

impl HasWindowHandle for &dyn PlatformWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        todo!()
    }
}

impl HasDisplayHandle for &dyn PlatformWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        todo!()
    }
}

impl<'w> Into<SurfaceTarget<'w>> for &dyn PlatformWindow {
    fn into(self) -> SurfaceTarget<'w> {
        todo!()
    }
}

pub trait PlatformEventLoop: Any {
    fn as_any(&self) -> &dyn Any;

    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait PlatformContext: Any {
    fn as_any(&self) -> &dyn Any;
}
