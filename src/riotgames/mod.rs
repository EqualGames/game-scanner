#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::macos::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
mod types;
pub mod utils;
#[cfg(target_os = "windows")]
mod windows;
mod yaml;
