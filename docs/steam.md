# Steam

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

- Windows
  `{Executable} -silent steam://run/<id>`

### Close game

- Windows
  `{Executable} -shutdown`

### Uninstall game

- Windows
  `{Executable} -silent steam://uninstall/<Manifest.appid>`
