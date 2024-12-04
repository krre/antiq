cfg_if::cfg_if! {
    if  #[cfg(linux)] {
        pub mod linux;
        #[allow(unused_imports)]
        pub use linux::*;
    } else if #[cfg(macos)] {
        pub mod macos;
        #[allow(unused_imports)]
        pub use macos::*;
    } else if #[cfg(windows)] {
        pub mod windows;
        #[allow(unused_imports)]
        pub use windows::*;
    } else {
        compile_error!("Platform not supported")
    }
}

pub trait PlatformEventLoop {
    fn run(&self);
}
