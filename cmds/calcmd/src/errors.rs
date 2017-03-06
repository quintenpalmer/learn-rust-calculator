use std::fmt;
use std::result;
use std::io;

use calclogic;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CalcLogicError(calclogic::Error),
    IOError(io::Error),
    ArgError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::CalcLogicError(ref cle) => write!(f, "calclogic error: {}", cle),
            Error::IOError(ref ioe) => write!(f, "io error: {}", ioe),
            Error::ArgError => write!(f, "must supply the file to read"),
        }
    }
}
