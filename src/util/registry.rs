use std::{env, io};

use winreg;

pub fn get_local_machine_reg_key(sub_key: &str) -> io::Result<winreg::RegKey> {
    let reg = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);

    let key = match env::consts::ARCH {
        "x86_64" => String::from("SOFTWARE\\WOW6432Node\\") + sub_key,
        _ => String::from("SOFTWARE\\") + sub_key,
    };

    return reg.open_subkey(key);
}

pub fn get_current_user_reg_key(sub_key: &str) -> io::Result<winreg::RegKey> {
    let reg = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);

    let key = match env::consts::ARCH {
        "x86_64" => String::from("SOFTWARE\\") + sub_key,
        _ => String::from("SOFTWARE\\") + sub_key,
    };

    return reg.open_subkey(key);
}
