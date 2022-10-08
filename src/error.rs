use std::error;
use std::fmt::Display;
use std::io;

pub type Res<A> = Result<A, TransposeError>;

#[derive(Debug)]
pub enum TransposeError {
    IoError(io::Error),
    Other(String),
}

impl error::Error for TransposeError {}

impl Display for TransposeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransposeError::Other(s) => writeln!(f, "{}", s),
            TransposeError::IoError(e) => writeln!(f, "{}", e),
        }
    }
}

impl From<io::Error> for TransposeError {
    fn from(e: io::Error) -> Self {
        TransposeError::IoError(e)
    }
}

impl From<&str> for TransposeError {
    fn from(s: &str) -> Self {
        TransposeError::Other(s.to_string())
    }
}
