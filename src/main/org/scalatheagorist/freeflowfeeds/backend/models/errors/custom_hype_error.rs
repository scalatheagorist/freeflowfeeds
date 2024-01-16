use hyper::Error;
use std::convert::Infallible;

#[derive(Debug)]
pub struct CustomHyperError(pub String);

impl std::fmt::Display for CustomHyperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Custom Hyper Error: {}", self.0)
    }
}

impl std::error::Error for CustomHyperError {}

impl From<CustomHyperError> for Error {
    #[allow(unreachable_code)]
    #[allow(unconditional_recursion)]
    fn from(_err: CustomHyperError) -> Self {
        Error::from(_err)
    }
}

impl From<CustomHyperError> for Infallible {
    #[allow(unreachable_code)]
    #[allow(unconditional_recursion)]
    fn from(_err: CustomHyperError) -> Self {
        Infallible::from(_err)
    }
}
