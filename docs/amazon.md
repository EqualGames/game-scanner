# Amazon

## Windows

### Launcher Info

- LAUNCHER_PATH
  ```
  %LocalAppData%\Amazon Games
  ```
- LAUNCHER_EXECUTABLE
  ```
  %LocalAppData%\Amazon Games\
  ```

### Games

The information about the games is located in `%LocalAppData%\Amazon Games\Data\Games\Sql\GameInstallInfo.sqlite`. To
collect this information is necessary connect using a sqlite driver, and using this query:

```sql
SELECT id, ProductTitle, InstallDirectory
FROM DbSet
WHERE Installed = 1;
```

### Start game

```commandline
LAUNCHER_EXECUTABLE amazon-games://play/GAME_ID
```


### Uninstall game 

```
"C:\\Amazon Games\\Library\\__InstallData__\\Amazon Game Remover.exe" -m Game -p GAME_ID
```
```
HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\AmazonGames/METAL SLUG 3\UninstallString
```
