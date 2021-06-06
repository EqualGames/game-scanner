use std::path::{Path, PathBuf};

use rusqlite::{Connection, OpenFlags, Row};

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::{Game, GameType},
};

pub fn read_all(file: &Path, launcher_executable: &Path) -> Result<Vec<Game>> {
    let conn =
        Connection::open_with_flags(&file, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid GOG manifest: {}", error.to_string()),
            )
        })?;

    let mut manifest_ids = Vec::new();

    {
        let mut stmt = conn
            .prepare("SELECT releaseKey FROM LibraryReleases WHERE releaseKey LIKE 'gog_%'")
            .map_err(|error| {
                Error::new(
                    ErrorKind::InvalidManifest,
                    format!(
                        "Error to read the GOG manifest (LibraryReleases): {}",
                        error.to_string()
                    ),
                )
            })?;

        let rows = stmt
            .query_map([], |row| parse_library_releases(row))
            .map_err(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => Error::new(
                    ErrorKind::LibraryNotFound,
                    format!("GOG library could be empty"),
                ),
                _ => Error::new(
                    ErrorKind::InvalidManifest,
                    format!(
                        "Error to read the GOG manifest (LibraryReleases): {}",
                        error.to_string()
                    ),
                ),
            })?;

        for id in rows {
            manifest_ids.push(id?.replace("gog_", ""));
        }
    }

    let mut games = Vec::<Game>::new();

    for id in manifest_ids {
        let mut stmt = conn
            .prepare("SELECT * FROM LimitedDetails WHERE productId = :id")
            .map_err(|error| {
                Error::new(
                    ErrorKind::InvalidManifest,
                    format!(
                        "Error to read the GOG manifest (LimitedDetails): {}",
                        error.to_string()
                    ),
                )
            })?;

        let mut game = stmt
            .query_row(&[(":id", &id)], |row| {
                parse_limited_details(row, launcher_executable)
            })
            .map_err(|error| {
                Error::new(
                    ErrorKind::InvalidManifest,
                    format!(
                        "Error to read the GOG manifest (LimitedDetails): {}",
                        error.to_string()
                    ),
                )
            })?;

        let path = conn
            .prepare("SELECT installationPath FROM InstalledBaseProducts WHERE productId = :id")
            .map_err(|error| {
                Error::new(
                    ErrorKind::InvalidManifest,
                    format!(
                        "Error to read the GOG manifest (Manifests): {}",
                        error.to_string()
                    ),
                )
            })
            .and_then(|mut statement| {
                statement
                    .query_row(&[(":id", &game.id)], parse_installed_base_product)
                    .map_err(|error| match error {
                        rusqlite::Error::QueryReturnedNoRows => Error::new(
                            ErrorKind::LibraryNotFound,
                            format!("GOG library could be empty"),
                        ),
                        _ => Error::new(
                            ErrorKind::InvalidManifest,
                            format!(
                                "Error to read the GOG manifest (Manifests): {}",
                                error.to_string()
                            ),
                        ),
                    })
            });

        match path {
            Ok(path) => {
                game.path = Some(path.clone());
                game.commands.launch = Some(vec![
                    launcher_executable.display().to_string(),
                    String::from("/command=runGame"),
                    format!("/gameId={}", &game.id),
                    format!("/path={}", &path.display().to_string()),
                ]);
                game.state.installed = true;
            }
            Err(error) => match error.kind() {
                ErrorKind::LibraryNotFound => {
                    game.state.installed = false;
                }
                _ => {
                    return Err(error);
                }
            },
        }

        games.push(game);
    }

    return Ok(games);
}

pub fn read(id: &str, file: &Path, launcher_executable: &Path) -> Result<Game> {
    let conn =
        Connection::open_with_flags(&file, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid GOG manifest: {}", error.to_string()),
            )
        })?;

    let mut stmt = conn
        .prepare("SELECT * FROM LimitedDetails WHERE productId = :id")
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error to read the GOG manifest (LimitedDetails): {}",
                    error.to_string()
                ),
            )
        })?;

    let mut game = stmt
        .query_row(&[(":id", &id)], |row| {
            parse_limited_details(row, launcher_executable)
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error to read the GOG manifest (LimitedDetails): {}",
                    error.to_string()
                ),
            )
        })?;

    let path = conn
        .prepare("SELECT installationPath FROM InstalledBaseProducts WHERE productId = :id")
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!(
                    "Error to read the GOG manifest (Manifests): {}",
                    error.to_string()
                ),
            )
        })
        .and_then(|mut statement| {
            statement
                .query_row(&[(":id", &game.id)], parse_installed_base_product)
                .map_err(|error| match error {
                    rusqlite::Error::QueryReturnedNoRows => Error::new(
                        ErrorKind::LibraryNotFound,
                        format!("GOG library could be empty"),
                    ),
                    _ => Error::new(
                        ErrorKind::InvalidManifest,
                        format!(
                            "Error to read the GOG manifest (Manifests): {}",
                            error.to_string()
                        ),
                    ),
                })
        });

    match path {
        Ok(path) => {
            game.path = Some(path.clone());
            game.commands.launch = Some(vec![
                launcher_executable.display().to_string(),
                String::from("/command=runGame"),
                format!("/gameId={}", &game.id),
                format!("/path={}", &path.display().to_string()),
            ]);

            if cfg!(target_os = "windows") {
                game.commands.uninstall =
                    Some(vec![path.join("unins000.exe").display().to_string()]);
            }

            game.state.installed = true;
        }
        Err(error) => match error.kind() {
            ErrorKind::LibraryNotFound => {
                game.state.installed = false;
            }
            _ => {
                return Err(error);
            }
        },
    }

    Ok(game)
}

fn parse_library_releases(row: &Row) -> rusqlite::Result<String> {
    let id = row.get::<_, String>(0)?;

    Ok(id)
}

fn parse_limited_details(row: &Row, launcher_executable: &Path) -> rusqlite::Result<Game> {
    let columns = row.column_count();

    let mut game = Game::default();
    game._type = GameType::GOG.to_string();

    for col in 0..columns {
        let name = row.column_name(col)?;

        match name {
            "productId" => game.id = row.get::<_, i64>(col)?.to_string(),
            "title" => game.name = row.get(col)?,
            _ => {}
        }
    }

    game.commands.install = Some(vec![
        launcher_executable.display().to_string(),
        String::from("/command=installGame"),
        format!("/gameId={}", &game.id),
    ]);

    Ok(game)
}

fn parse_installed_base_product(row: &Row) -> rusqlite::Result<PathBuf> {
    row.get::<_, String>(0).map(PathBuf::from)
}
