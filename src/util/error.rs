use std::io;

pub fn make_io_error(message: &str) -> io::Error {
    return io::Error::new(io::ErrorKind::Other, message);
}
