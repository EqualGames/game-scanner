# Epic Games

## Windows

### Launcher Info

- LAUNCHER_EXECUTABLE
  `HKEY_CURRENT_USER\SOFTWARE\Epic Games\EOS\ModSdkCommand`
- LAUNCHER_PATH:
  `HKEY_CURRENT_USER\SOFTWARE\Epic Games\EpicGamesLauncher\AppDataPath`

### Games

- LAUNCHER_MANIFESTS:
  ```
  LAUNCHER_PATH\Manifests\*.item
  ```

### Start game

```commandline
LAUNCHER_EXECUTABLE com.epicgames.launcher://apps/MANIFEST_APP_NAME?action=launch&silent=true
```

### Show library

```commandline
LAUNCHER_EXECUTABLE com.epicgames.launcher://store/library
```
