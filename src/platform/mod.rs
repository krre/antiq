#[cfg(target_os = "linux")]
mod linux;
pub use linux::*;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;
