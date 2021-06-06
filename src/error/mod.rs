use std::{error, fmt, io};

use chrono::Utc;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self::_new(kind, error.into())
    }

    fn _new(kind: ErrorKind, error: Box<dyn error::Error + Send + Sync>) -> Self {
        Error { kind, error }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Error {
        Error::new(ErrorKind::SQLite, error)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Error {
        Error::new(ErrorKind::YAML, error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::new(ErrorKind::JSON, error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::new(ErrorKind::IO, error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(fmt)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.error.source()
    }

    #[allow(deprecated, deprecated_in_future)]
    fn description(&self) -> &str {
        self.error.description()
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn error::Error> {
        self.error.cause()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    IgnoredApp,
    InvalidGame,
    InvalidLauncher,
    InvalidManifest,
    InvalidLibrary,
    GameNotFound,
    GameProcessNotFound,
    LauncherNotFound,
    LibraryNotFound,
    IO,
    SQLite,
    JSON,
    YAML,
    WinReg,
    Other,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn can_logger(error: &Error) -> bool {
    return error.kind().ne(&ErrorKind::IgnoredApp)
        && error.kind().ne(&ErrorKind::LauncherNotFound)
        || cfg!(debug_assertions)
        || cfg!(test);
}

#[allow(unused)]
pub fn print_error(error: &Error) {
    if can_logger(error) {
        println!(
            "[{}][{:?}] {}",
            Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f%:z"),
            error.kind(),
            error.to_string(),
        );
    }
}
