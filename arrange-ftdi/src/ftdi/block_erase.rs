use core::fmt;

#[derive(Clone, Copy, Debug)]
pub enum BlockErase {
    FourK = 4,
    ThirtyTwoK = 32,
    SixtyFourK = 64,
}

impl fmt::Display for BlockErase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self as usize).to_string())
    }
}
