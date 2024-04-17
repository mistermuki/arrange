use core::fmt;

use super::error::GenericArgumentError;

pub fn test_mode_parser(arg: &str) -> Result<TestMode, GenericArgumentError> {
    match arg {
        "0" => Ok(TestMode::NoTest),
        "1" => Ok(TestMode::Normal),
        "2" => Ok(TestMode::Quad),
        _ => Err(GenericArgumentError::new(
            "Only valid values are 0, 1, or 2",
        )),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TestMode {
    NoTest,
    Normal,
    Quad,
}

impl fmt::Display for TestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestMode::NoTest => write!(f, "0"),
            TestMode::Normal => write!(f, "1"),
            TestMode::Quad => write!(f, "2"),
        }
    }
}
