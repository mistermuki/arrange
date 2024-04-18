use core::fmt;
use std::error::Error;

#[derive(Clone, Debug, Default)]
pub struct GenericArgumentError {
    display: String,
}

impl GenericArgumentError {
    pub fn new(display: &str) -> Self {
        Self {
            display: display.to_string(),
        }
    }
}

impl fmt::Display for GenericArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl Error for GenericArgumentError {}

