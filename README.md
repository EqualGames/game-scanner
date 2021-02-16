# Game Scanner

Game Scanner for any launcher and OS.

## Requirements

- [Rust](https://www.rust-lang.org)
    - cargo >= v1.49.0
    - rustup >= v1.23.1
    - rustc >= v1.49.0

## Launchers Support

| Launcher   | Multi-directories[¹](#multi-directories) | Windows | Linux | MacOS |
| ---------- | ---------------------------------------- | ------- | ----- | ----- |
| Amazon     | ✅                                        | ✅       | ❌     | ❌     |
| Blizzard   | ❓                                        | ✅       | ❌     | ❌     |
| Epic Games | ❌                                        | ✅       | ❌     | ❌     |
| GOG        | ❌                                        | ✅       | ❌     | ❌     |
| Origin     | ❌                                        | ✅       | ❌     | ❌     |
| Riot Games  | ❓                                        | ✅       | ❌     | ❌     |
| Steam      | ✅                                        | ✅       | ❌     | ❌     |
| Ubisoft    | ❌                                        | ✅       | ❌     | ❌     |

<a name="multi-directories"></a>[1]: **Multi-directories**: is different game install locations (e.g., folders, and
drivers).

## NodeJS Binding

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
