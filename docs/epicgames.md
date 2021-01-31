# Epic Games

## Windows

### Launcher Info

- Executable
  `HKEY_CURRENT_USER\SOFTWARE\Epic Games\EOS\ModSdkCommand`
- Path:
  `HKEY_CURRENT_USER\SOFTWARE\Epic Games\EpicGamesLauncher\AppDataPath`
- Manifests:
  `{Path}\Manifests\*.item`

### Start game

`{Executable} com.epicgames.launcher://apps/{Manifest.AppName}?action=launch&silent=true`


