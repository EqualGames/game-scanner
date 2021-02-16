# GOG

## Windows

### Launcher info

- LAUNCHER_PATH:
  ```
  HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\GOG.com\GalaxyClient\paths\client
  ```
- LAUNCHER_EXECUTABLE:
  ```
  HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\GOG.com\GalaxyClient\clientExecutable
  ```
- LAUNCHER_GAMES:
  ```
  HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\GOG.com\Games
  ```

### Start game

```commandline
LAUNCHER_EXECUTABLE /command=runGame /gameId=GAME_ID /path=GAME_INSTALL_LOCATION
```

### Game view

```commandline
LAUNCHER_EXECUTABLE goggalaxy://openGameView/GAME_ID
```

### Shutdown

```commandline
LAUNCHER_EXECUTABLE /command=shutdown
```
