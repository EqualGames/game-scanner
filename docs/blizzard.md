# Blizzard

## Windows

### Launcher Info

- LAUNCHER_EXECUTABLE:
  ```
  HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Battle.net\C:\Program Files (x86)\Battle.net\DisplayIcon
  ```

### Games

The information about the games installed is located in `C:\ProgramData\Battle.net\Agent\product.db`. To collect this
information is necessary to use a protocol buffers (proto3), following
the [scheme](../src/blizzard/proto/product_db.proto).

#### Game codes:

| Game                                  | UID         |  Code            |
| ------------------------------------- | ----------- |  --------------- |
| Star Craft                            | s1          |  S1              |
| Star Craft 2                          | s2          |  S2              |
| World of Warcraft                     | wow         |  WoW             |
| World of Warcraft Classic             | wow_classic |  WoW_wow_classic |
| Overwatch                             | prometheus  |  Pro             |
| Warcraft III                          | w3          |  W3              |
| Hearthstone                           | hs_beta     |  WTCG            |
| Heroes of the Storm                   | heroes      |  Hero            |
| Diablo III                            | diablo3     |  D3              |
| Call of Duty: Black Ops 4             | viper       |  VIPR            |
| Call of Duty: Modern Warfare          | odin        |  ODIN            |
| Call of Duty: MW2 Campaign Remastered | lazarus     |  LAZR            |
| Call of Duty: Black Ops Cold War      | zeus        |  ZEUS            |

### Start game

```commandline
LAUNCHER_EXECUTABLE --exec="launch GAME_CODE"
```

### Close launcher

Closes only the launcher, the game continues running.

```commandline
LAUNCHER_EXECUTABLE --exec=shutdown
```

### Uninstall launcher

To get the uninstall command:

```
HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Battle.net\UninstallString
```
