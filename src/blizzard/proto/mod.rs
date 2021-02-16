use crate::error::{Error, ErrorKind, Result};
use prost::Message;
use std::fs;
use std::path::Path;

pub mod product;

pub fn read_product(database_path: &Path) -> Result<product::Database> {
    let bytes = fs::read(database_path)
        .map(bytes::Bytes::from)
        .map_err(|error| {
            Error::new(
                ErrorKind::InvalidLauncher,
                format!(
                    "Invalid Blizzard database: {} {}",
                    database_path.display().to_string(),
                    error.to_string()
                ),
            )
        })
        .unwrap();

    let data: std::result::Result<product::Database, prost::DecodeError> =
        product::Database::decode(bytes);

    if data.is_err() {
        return Err(Error::new(
            ErrorKind::InvalidLauncher,
            format!(
                "Error on read the Blizzard database: {} {}",
                database_path.display().to_string(),
                data.err().unwrap().to_string()
            ),
        ));
    }

    return Ok(data.unwrap());
}
