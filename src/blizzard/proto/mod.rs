use prost::Message;
use std::fs;
use std::io;
use std::path::Path;

pub mod product;

pub fn read_product(database_path: &Path) -> io::Result<product::Database> {
    return fs::read(database_path)
        .map(bytes::Bytes::from)
        .and_then(|bytes| product::Database::decode(bytes).map_err(io::Error::from));
}
