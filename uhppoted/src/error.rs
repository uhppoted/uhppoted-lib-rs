use std::fmt;

#[derive(Debug)]
pub struct Error {
    _kind: ErrorKind,
    message: String,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    _NoResponse,
    Timeout,
    IO,
    Other,
}

impl std::error::Error for Error {}

impl Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            _kind: ErrorKind::IO,
            message: format!("{}", err),
        }
    }
}
