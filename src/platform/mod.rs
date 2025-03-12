use std::any::Any;

use wgpu::SurfaceTargetUnsafe;

use crate::{
    core::{
        Border2D, Pos2D, Result, Size2D,
        event::{Event, EventHandler},
    },
    window::WindowId,
};

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
        compile_error!("Platform is not supported");
    }
}

pub trait PlatformApplication: Any {
    fn as_any(&self) -> &dyn Any;
}

pub trait PlatformWindow: Any {
    fn as_any(&self) -> &dyn Any;

    fn id(&self) -> WindowId;

    fn surface_target(&self) -> SurfaceTargetUnsafe;

    fn set_title(&self, title: &str);

    fn set_visible(&self, visible: bool);

    fn set_position(&self, pos: Pos2D);

    fn set_size(&self, size: Size2D);

    fn border(&self) -> Border2D {
        Border2D::default()
    }
}

pub trait PlatformEventLoop: Any {
    fn as_any(&self) -> &dyn Any;

    fn process_events(&self, event_handler: Box<dyn EventHandler>) -> Result<()>;

    fn send_event(&self, event: Box<dyn Event>);

    fn quit(&self);
}
