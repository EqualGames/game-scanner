use crate::error::{Error, ErrorKind, Result};
use crate::prelude::Game;
use rusqlite::{params, Connection, OpenFlags};
use std::path::Path;

pub fn read(file: &Path, launcher_path: &Path) -> Result<Vec<Game>> {
    let conn = Connection::open_with_flags(&file, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!("Invalid Amazon Games database: {}", error.to_string()),
            )
        })
        .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, ProductTitle, InstallDirectory FROM DbSet WHERE Installed = 1;")
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Error to read the Amazon Games database: {}",
                    error.to_string()
                ),
            )
        })
        .unwrap();

    return stmt
        .query_map(params![], |row| {
            let id: String = row.get(0).map_err(Error::from).unwrap();

            return Ok(Game {
                _type: String::from("amazon"),
                id: id.clone(),
                name: row.get(1).map_err(Error::from).unwrap(),
                path: row.get(2).map_err(Error::from).unwrap(),
                launch_command: vec![
                    launcher_path
                        .join("App")
                        .join("Amazon Games.exe")
                        .display()
                        .to_string(),
                    format!("amazon-games://play/{}", id),
                ],
            });
        })
        .map(|data| {
            data.into_iter()
                .map(|item| item.map_err(Error::from).unwrap())
                .collect::<Vec<Game>>()
        })
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Error to read the Amazon Games manifest: {}",
                    error.to_string()
                ),
            )
        });
}
