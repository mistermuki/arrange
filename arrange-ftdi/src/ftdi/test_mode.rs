use core::fmt;

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
