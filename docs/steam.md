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
`{Executable} -silent steam://launch/<Manifest.appid>`

### Close launcher

`{Executable} -shutdown`

### Install game

`{Executable} -silent steam://install/<Manifest.appid>`

### Uninstall game

`{Executable} -silent steam://uninstall/<Manifest.appid>`
