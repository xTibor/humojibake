use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    UnsupportedEncoding { encoding_name: String },
    IoError(io::Error),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::UnsupportedEncoding { .. } => None,
            Error::IoError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::UnsupportedEncoding { ref encoding_name } => {
                write!(f, "Unsupported encoding specified (\"{}\")", encoding_name)
            }
            Error::IoError(_) => {
                write!(f, "I/O error")
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}
