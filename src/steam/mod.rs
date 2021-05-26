#[cfg(target_os = "windows")]
pub use self::windows::{find, games};

mod acf;
mod types;
mod vdf;
#[cfg(target_os = "windows")]
mod windows;
