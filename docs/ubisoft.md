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
`{Executable}\upc.exe uplay://open/game/<install_reg_name>`

### Install game

`{Executable}\upc.exe uplay://install/<install_reg_name>`

### Uninstall game

`{Executable}\upc.exe uplay://uninstall/<install_reg_name>`
