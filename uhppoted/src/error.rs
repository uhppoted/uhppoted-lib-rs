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
    InvalidAddress,
    BindToReservedPort,
    BoundToReservedPort,
    InvalidResponse,
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

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            _kind: kind,
            message: format!("{}", kind),
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Error {
        Error {
            _kind: ErrorKind::Other,
            message: message.to_string(),
        }
    }
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match *self {
            InvalidAddress => "Invalid IPv4 address",
            BindToReservedPort => "Bind address:port reserved for broadcast",
            BoundToReservedPort => "Bound address:port reserved for broadcast",
            _NoResponse => "No response to request",
            Timeout => "Timeout waiting for response from controller",
            InvalidResponse => "Invalid response from controller",
            IO => "I/O",
            Other => "Other",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}
