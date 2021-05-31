# Game Scanner

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/EqualGames/game-scanner/Release)
![Crates.io](https://img.shields.io/crates/v/game-scanner)
![npm (scoped)](https://img.shields.io/npm/v/@equal-games/game-scanner)

Game Scanner for any launcher and OS.

- [Examples](./examples)
- [Benches](./benches)
- [Tests](./tests)

## Data structure

You can find the Rust data structure in [prelude.rs](./src/prelude.rs), and the Javascript data structure
in [index.d.ts](./node/lib/index.d.ts).

## Launchers Support

### OS

| Launcher   | Multi-directories[¹](#multi-directories) | Windows | Linux | MacOS |
| ---------- | ---------------------------------------- | ------- | ----- | ----- |
| Amazon     | ✅                                        | ✅       | ❌     | ❌     |
| Blizzard   | ❓                                        | ✅       | ❌     | ❌     |
| Epic Games | ❌                                        | ✅       | ❌     | ❌     |
| GOG        | ❌                                        | ✅       | ❌     | ❌     |
| Origin     | ❌                                        | ✅       | ❌     | ❌     |
| Riot Games | ❓                                        | ✅       | ❌     | ❌     |
| Steam      | ✅                                        | ✅       | ❌     | ❌     |
| Ubisoft    | ❌                                        | ✅       | ❌     | ❌     |

<a name="multi-directories"></a>[1]: **Multi-directories**: is different game install locations (e.g., folders, and
drivers).

### Game Commands support

| Launcher   | Install | Launch | Uninstall |
| ---------- | ------- | ------ | --------- |
| Amazon     | ❌       | ✅      | ❌         |
| Blizzard   | ❌       | ✅      | ❌         |
| Epic Games | ❌       | ✅      | ❌         |
| GOG        | ❌       | ✅      | ❌         |
| Origin     | ✅       | ✅      | ❌         |
| Riot Games | ❌       | ✅      | ✅         |
| Steam      | ✅       | ✅      | ✅         |
| Ubisoft    | ✅       | ✅      | ✅         |

### Game State support

| Launcher   | Installed | Needs Update | Downloading | Total Bytes | Received Bytes |
| ---------- | --------- | ------------ | ----------- | ----------- | -------------- |
| Amazon     | ❌         | ❌            | ❌           | ❌           | ❌              |
| Blizzard   | ❌         | ❌            | ❌           | ❌           | ❌              |
| Epic Games | ❌         | ❌            | ❌           | ❌           | ❌              |
| GOG        | ❌         | ❌            | ❌           | ❌           | ❌              |
| Origin     | ❌         | ❌            | ❌           | ❌           | ❌              |
| Riot Games | ❌         | ❌            | ❌           | ❌           | ❌              |
| Steam      | ✅         | ✅            | ✅           | ✅           | ✅              |
| Ubisoft    | ❌         | ❌            | ❌           | ❌           | ❌              |

### Operations

| Launcher   | List Games | Find Game[¹](#find-game) |
| ---------- | ---------- | ------------------------ |
| Amazon     | ✅          | ✅                        |
| Blizzard   | ✅          | ✅                        |
| Epic Games | ✅          | ✅                        |
| GOG        | ✅          | ✅                        |
| Origin     | ✅          | ✅                        |
| Riot Games | ✅          | ✅                        |
| Steam      | ✅          | ✅                        |
| Ubisoft    | ✅          | ✅                        |

<a name="find-game"></a>[1]: **Find Game**: you can find for a specific game passing only the `id`.

### Management

| Launcher   | Launch | Get Processes | Close |
| ---------- | ------ | ------------- | ----- |
| Amazon     | ✅      | ❓             | ❓     |
| Blizzard   | ✅      | ❓             | ❓     |
| Epic Games | ✅      | ❓             | ❓     |
| GOG        | ✅      | ❓             | ❓     |
| Origin     | ✅      | ❓             | ❓     |
| Riot Games | ✅      | ❓             | ❓     |
| Steam      | ✅      | ✅             | ✅     |
| Ubisoft    | ✅      | ❓             | ❓     |

<a name="get-processes"></a>[1]: **Game Processes**: return a list with all `Id` or `PID` of processes from a specific
game.

## Requirements

- [Rust](https://www.rust-lang.org)
    - cargo >= v1.49.0
    - rustup >= v1.23.1
    - rustc >= v1.49.0

## NodeJS Binding

- [Examples](./node/examples)

### Requirements

- [Visual Studio](https://visualstudio.microsoft.com/) >= 2019
    - Desktop Development with C++
        - MSVC >= v142
        - Windows 10 SDK >= 10.0.18362.0
- [Node](https://nodejs.org)
    - node \>= 12.20.0
    - npm \>= 6.14.8
- [windows-build-tools](https://neon-bindings.com/docs/getting-started#install-node-build-tools) >= 5.2.2
    - `npm config set msvs_version 2019`
    - `npm config set python python2.7`

### Resources

- [Neon Bindings](https://neon-bindings.com)

## Code of Conduct

If you are interested in contributing to the project, please take a look at the [Code of Conduct](./CODE_OF_CONDUCT.md).

## License

This project is licensed under the terms of the
[MIT license](./LICENSE).
