use std::env;

use winreg::{
    self,
    enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
    RegKey,
};

use crate::error::{Error, ErrorKind, Result};

pub fn get_local_machine_reg_key(sub_key: &str) -> Result<RegKey> {
    let reg = RegKey::predef(HKEY_LOCAL_MACHINE);

    let key = match env::consts::ARCH {
        "x86_64" => String::from("SOFTWARE\\WOW6432Node\\") + sub_key,
        _ => String::from("SOFTWARE\\") + sub_key,
    };

    reg.open_subkey(key)
        .map_err(|error| Error::new(ErrorKind::WinReg, error))
}

pub fn get_current_user_reg_key(sub_key: &str) -> Result<RegKey> {
    let reg = RegKey::predef(HKEY_CURRENT_USER);
    let key = String::from("SOFTWARE\\") + sub_key;

    reg.open_subkey(key)
        .map_err(|error| Error::new(ErrorKind::WinReg, error))
}

pub fn get_sub_key(reg: &RegKey, key: &str) -> Result<RegKey> {
    reg.open_subkey(key)
        .map_err(|error| Error::new(ErrorKind::WinReg, error))
}

pub fn get_value(reg: &RegKey, key: &str) -> Result<String> {
    reg.get_value::<String, &str>(key)
        .map_err(|error| Error::new(ErrorKind::WinReg, error))
}
