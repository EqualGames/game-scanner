use bytes::Bytes;
use prost::Message;
use std::{fs, io::*, path::*};

pub mod product;

pub fn read_product(database_path: &Path) -> Result<product::Database> {
    let content = fs::read(database_path).map_err(Error::from)?;
    let bytes = Bytes::from(content);

    return product::Database::decode(bytes).map_err(Error::from);
}
