use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    OutOfRangeError { min: i32, max: i32, actual: i32 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::IoError(io_error) => write!(f, "IoError: {}", io_error),
            Error::OutOfRangeError { min, max, actual } => write!(
                f,
                "OutOfRangeError: min: {}, max: {}, actual: {}",
                min, max, actual
            ),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl error::Error for Error {}
