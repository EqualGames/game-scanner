# Ubisoft

## Windows

### Launcher Info

- Executable:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Ubisoft\Launcher\InstallDir`
- Games:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Ubisoft\Launcher\Installs`
- Game Name:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\\Uninstall\Uplay Install <install_identifier>\DisplayName`
- Game Path:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\\Uninstall\Uplay Install <install_identifier>\InstallLocation`

### Start game

`{Executable}\upc.exe uplay://launch/<install_reg_name>`
