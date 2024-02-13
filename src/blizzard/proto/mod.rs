use std::{
    fs,
    io::{Error, Result},
    path::Path,
};

use bytes::Bytes;
use prost::Message;

#[allow(clippy::all)]
#[allow(clippy::nursery)]
#[allow(clippy::pedantic)]
pub mod product;

pub fn read_product(database_path: &Path) -> Result<product::Database> {
    let content = fs::read(database_path).map_err(Error::from)?;
    let bytes = Bytes::from(content);

    product::Database::decode(bytes).map_err(Error::from)
}
