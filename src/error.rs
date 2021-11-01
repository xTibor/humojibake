use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    UnsupportedEncoding { encoding_name: String },
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::UnsupportedEncoding { ref encoding_name } => {
                write!(f, "Unsupported encoding specified (\"{}\")", encoding_name)
            }
        }
    }
}
