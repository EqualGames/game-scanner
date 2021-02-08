# Origin

## Windows

### Launcher Info

- Executable:
  `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Origin\ClientPath`
- Manifests:
  `C:\ProgramData\Origin\LocalContent\**\*.mfst`

### Start game

`{Executable} origin2://game/launch?offerIds=<Manifest.id>`
`{Executable} origin2://game/launch?offerIds=<Manifest.id>&autoDownload=true`### Start game

### Install game
`origin2://game/download?offerId=<Manifest.id>`

### Close launcher

`{Executable} origin://quit`


