# Steam

## Windows

### Launcher Info

- Executable:
  `HKEY_CURRENT_USER\SOFTWARE\Valve\Steam\SteamExe`
- InstallPath:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Valve\Steam`
- Path:
  `HKEY_CURRENT_USER\SOFTWARE\Valve\Steam\SteamPath`
- Manifests:
  `{Path|InstallPath}\steamapps\*.acf`

### Start game

`{Executable} -silent steam://run/<Manifest.appid>`

### Close game

`{Executable} -shutdown`

### Uninstall game

`{Executable} -silent steam://uninstall/<Manifest.appid>`
