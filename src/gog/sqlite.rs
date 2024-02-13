use std::path::{Path, PathBuf};

use rusqlite::{params_from_iter, Connection, OpenFlags, Row};

use crate::{
    error::{Error, ErrorKind, Result},
    prelude::{Game, GameType},
    utils::sqlite::repeat_vars,
};

#[allow(clippy::too_many_lines)]
pub fn read_all(file: &Path, launcher_executable: &Path) -> Result<Vec<Game>> {
    let conn =
        Connection::open_with_flags(file, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid GOG manifest: {error}"),
            )
        })?;

    let mut manifest_ids = Vec::new();

    {
        let mut stmt = conn
            .prepare("SELECT releaseKey FROM LibraryReleases WHERE releaseKey LIKE 'gog_%'")
            .map_err(|error| {
                Error::new(
                    ErrorKind::InvalidManifest,
                    format!("Error to read the GOG manifest (LibraryReleases): {error}"),
                )
            })?;

        let rows = stmt
            .query_map([], parse_library_releases)
            .map_err(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => Error::new(
                    ErrorKind::LibraryNotFound,
                    "GOG library could be empty".to_string(),
                ),
                _ => Error::new(
                    ErrorKind::InvalidManifest,
                    format!("Error to read the GOG manifest (LibraryReleases): {error}"),
                ),
            })?;

        for id in rows {
            manifest_ids.push(id?.replace("gog_", ""));
        }
    }

    let mut stmt = conn
        .prepare(
            format!(
                "SELECT * FROM LimitedDetails WHERE productId IN ({})",
                repeat_vars(manifest_ids.len())
            )
            .as_str(),
        )
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Error to read the GOG manifest (LimitedDetails): {error}"),
            )
        })?;

    let game_columns = stmt
        .column_names()
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

    let manifest_games = stmt
        .query_map(params_from_iter(manifest_ids.iter()), |row| {
            parse_limited_details(&game_columns, row, launcher_executable)
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Error to read the GOG manifest (LimitedDetails): {error}"),
            )
        })?;

    let mut games = Vec::<Game>::new();

    for game in manifest_games {
        let mut game = game?;

        let path = conn
            .prepare("SELECT installationPath FROM InstalledBaseProducts WHERE productId = :id")
            .and_then(|mut statement| {
                statement.query_row(&[(":id", &game.id)], parse_installed_base_product)
            })
            .map_err(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => Error::new(
                    ErrorKind::LibraryNotFound,
                    "GOG library could be empty".to_string(),
                ),
                _ => Error::new(
                    ErrorKind::InvalidManifest,
                    format!("Error to read the GOG manifest (Manifests): {error}"),
                ),
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

    Ok(games)
}

pub fn read(id: &str, file: &Path, launcher_executable: &Path) -> Result<Game> {
    let conn =
        Connection::open_with_flags(file, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Invalid GOG manifest: {error}"),
            )
        })?;

    let mut stmt = conn
        .prepare("SELECT * FROM LimitedDetails WHERE productId = :id")
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Error to read the GOG manifest (LimitedDetails): {error}"),
            )
        })?;

    let game_columns = stmt
        .column_names()
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut game = stmt
        .query_row(&[(":id", &id)], |row| {
            parse_limited_details(&game_columns, row, launcher_executable)
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Error to read the GOG manifest (LimitedDetails): {error}"),
            )
        })?;

    let path = conn
        .prepare("SELECT installationPath FROM InstalledBaseProducts WHERE productId = :id")
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidManifest,
                format!("Error to read the GOG manifest (Manifests): {error}"),
            )
        })
        .and_then(|mut statement| {
            statement
                .query_row(&[(":id", &game.id)], parse_installed_base_product)
                .map_err(|error| match error {
                    rusqlite::Error::QueryReturnedNoRows => Error::new(
                        ErrorKind::LibraryNotFound,
                        "GOG library could be empty".to_string(),
                    ),
                    _ => Error::new(
                        ErrorKind::InvalidManifest,
                        format!("Error to read the GOG manifest (Manifests): {error}"),
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
    row.get::<_, String>(0)
}

fn parse_limited_details(
    columns: &[String],
    row: &Row,
    launcher_executable: &Path,
) -> rusqlite::Result<Game> {
    let mut game = Game {
        type_: GameType::GOG.to_string(),
        ..Default::default()
    };

    for col in 0..columns.len() {
        match columns.get(col).unwrap().as_str() {
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
