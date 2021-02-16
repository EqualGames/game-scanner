# Steam

## Windows

### Launcher Info

- LAUNCHER_EXECUTABLE:
  `HKEY_CURRENT_USER\SOFTWARE\Valve\Steam\SteamExe`
- LAUNCHER_PATH:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Valve\Steam`

### Games

- Manifests:
  `LAUNCHER_PATH\steamapps\*.acf`

### Start game

```commandline
LAUNCHER_EXECUTABLE -silent steam://run/GAME_ID
```

```commandline
LAUNCHER_EXECUTABLE -silent steam://rungameid/GAME_ID
```

```commandline
LAUNCHER_EXECUTABLE -silent steam://launch/GAME_ID
```

### Close launcher

Closes too the game

```commandline
LAUNCHER_EXECUTABLE -shutdown
```

### Install game

```commandline
LAUNCHER_EXECUTABLE -silent steam://install/GAME_ID
```

### Uninstall game

```commandline
LAUNCHER_EXECUTABLE -silent steam://uninstall/GAME_ID
```
