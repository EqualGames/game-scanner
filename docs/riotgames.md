# Riot Games

## Windows

### Launcher info

C:\ProgramData\Riot Games\

C:\ProgramData\Riot Games\RiotClientInstalls.json

    [f"--launch-product={game_id}", "--launch-patchline=live"],

# Riot Games

## Windows

### Launcher Info

- LAUNCHER_DATA_PATH:
  ```
  C:\ProgramData\Riot Games\
  ```
- LAUNCHER_INSTALLS:
  ```
  LAUNCHER_DATA_PATH\RiotClientInstalls.json
  ```
- LAUNCHER_EXECUTABLE:
  ```javascript
  LAUNCHER_INSTALLS.associated_client[MANIFEST_PRODUCT_INSTALL_FULL_PATH]
  ```

### Games

- Manifests:
    ```
    LAUNCHER_DATA_PATH\Metadata\**\*.product_settings.yaml
    ```

#### Game codes:

| Game                 | Code              | Server | Metadata Folder        |
| -------------------- | ----------------- | ------ | ---------------------- |
| League Of Legends    | league_of_legends | live   | league_of_legends.live |
| Legends Of Runeterra | bacon             | live   | bacon.live             |
| Valorant             | valorant          | live   | valorant.live          |

### Start game

```commandline
LAUNCHER_EXECUTABLE --launch-product=GAME_CODE --launch-patchline=GAME_SERVER
```

### Uninstall game

```commandline
LAUNCHER_EXECUTABLE --uninstall-product=GAME_CODE --uninstall-patchline=GAME_SERVER
```
