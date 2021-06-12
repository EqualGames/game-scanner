use std::env;

use winreg;

use crate::error::{Error, ErrorKind, Result};

pub fn get_local_machine_reg_key(sub_key: &str) -> Result<winreg::RegKey> {
    let reg = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);

    let key = match env::consts::ARCH {
        "x86_64" => String::from("SOFTWARE\\WOW6432Node\\") + sub_key,
        _ => String::from("SOFTWARE\\") + sub_key,
    };

    return reg
        .open_subkey(key)
        .map_err(|error| Error::new(ErrorKind::WinReg, error));
}

pub fn get_current_user_reg_key(sub_key: &str) -> Result<winreg::RegKey> {
    let reg = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);

    let key = match env::consts::ARCH {
        "x86_64" => String::from("SOFTWARE\\") + sub_key,
        _ => String::from("SOFTWARE\\") + sub_key,
    };

    return reg
        .open_subkey(key)
        .map_err(|error| Error::new(ErrorKind::WinReg, error));
}

pub fn get_sub_key(reg: &winreg::RegKey, key: &str) -> Result<winreg::RegKey> {
    return reg
        .open_subkey(key)
        .map_err(|error| Error::new(ErrorKind::WinReg, error));
}

pub fn get_value(reg: &winreg::RegKey, key: &str) -> Result<String> {
    return reg
        .get_value::<String, &str>(key)
        .map_err(|error| Error::new(ErrorKind::WinReg, error));
}
