#[cfg(target_os = "windows")]
pub use self::windows::*;

mod sqlite;
#[cfg(target_os = "windows")]
mod windows;
