use chrono::Utc;
use std::io;

pub fn make_io_error(message: &str) -> io::Error {
    return io::Error::new(io::ErrorKind::Other, message);
}

pub fn can_logger(error: &io::Error) -> bool {
    return error.kind().ne(&io::ErrorKind::InvalidData) || cfg!(debug_assertions);
}

pub fn print_error(error: &io::Error) -> () {
    if can_logger(error) {
        println!(
            "[{}] {}",
            Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f%:z"),
            error.to_string()
        );
    }
}
