#[cfg(target_os = "windows")]
pub use self::windows::*;

mod db;
mod proto;
mod types;
#[cfg(target_os = "windows")]
mod windows;
