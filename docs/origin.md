# Origin

## Windows

### Launcher Info

- LAUNCHER_EXECUTABLE:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Origin\ClientPath`

### Games

- Manifests:
  `C:\ProgramData\Origin\LocalContent\**\*.mfst`

### Start game

```commandline
LAUNCHER_EXECUTABLE origin2://game/launch?offerIds=GAME_ID
```

```commandline
LAUNCHER_EXECUTABLE origin2://game/launch?offerIds=GAME_ID&autoDownload=true
```

### Install game

```commandline
LAUNCHER_EXECUTABLE origin2://game/download?offerId=GAME_ID
```

### Close launcher

```commandline
LAUNCHER_EXECUTABLE origin://quit
```

### Show library

```commandline
LAUNCHER_EXECUTABLE origin2://library/open
```


