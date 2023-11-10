use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    pub(crate) message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {}", self.message)
    }
}

impl Error for CustomError {}
