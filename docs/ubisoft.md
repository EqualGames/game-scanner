# Ubisoft

## Windows

### Launcher Info

- LAUNCHER_PATH:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Ubisoft\Launcher\InstallDir`

### Game

- Games:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Ubisoft\Launcher\Installs`
- Game Name:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\\Uninstall\Uplay Install <install_identifier>\DisplayName`
- Game Path:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\\Uninstall\Uplay Install <install_identifier>\InstallLocation`

### Start game

```commandline
LAUNCHER_PATH\upc.exe uplay://launch/GAME_ID/0
```

### Game view

```commandline
LAUNCHER_PATH\upc.exe uplay://open/game/GAME_ID/0
```

### Install game

```commandline
LAUNCHER_PATH\upc.exe uplay://install/GAME_ID
```

### Uninstall game

```commandline
LAUNCHER_PATH\upc.exe uplay://uninstall/GAME_ID
```
