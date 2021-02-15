use crate::util::error::make_io_error;
use prost::{DecodeError, Message};
use std::fs;
use std::io;
use std::path::Path;

pub mod product;

pub fn read_product(database_path: &Path) -> io::Result<product::Database> {
    return fs::read(database_path)
        .map(bytes::Bytes::from)
        .and_then(|data| {
            product::Database::decode(data)
                .map_err(|error: DecodeError| make_io_error(&error.to_string()))
        });
}
