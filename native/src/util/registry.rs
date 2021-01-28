use winreg;
use std::env;

pub fn get_local_machine_reg_key(sub_key: &str) -> std::io::Result<winreg::RegKey> {
  let reg = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);

  let mut key = match env::consts::ARCH {
    "x86_64" => "SOFTWARE\\WOW6432Node\\".to_string(),
    _ => "SOFTWARE\\".to_string()
  };

  key.push_str(sub_key);

  return reg.open_subkey(key);
}

pub fn get_current_user_reg_key(sub_key: &str) -> std::io::Result<winreg::RegKey> {
  let reg = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);

  let mut key = match env::consts::ARCH {
    "x86_64" => "SOFTWARE\\".to_string(),
    _ => "SOFTWARE\\".to_string()
  };

  key.push_str(sub_key);

  return reg.open_subkey(key);
}