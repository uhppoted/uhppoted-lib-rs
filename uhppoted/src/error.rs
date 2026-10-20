#[derive(Debug)]
pub struct Error {
    _kind: ErrorKind,
    // message: String,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    _NoResponse,
    Timeout,
    IO,
    Other,
}
