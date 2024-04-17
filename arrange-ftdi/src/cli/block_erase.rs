use core::fmt;

use super::error::GenericArgumentError;

pub fn block_erase_parser(arg: &str) -> Result<BlockErase, GenericArgumentError> {
    match arg {
        "4" => Ok(BlockErase::FourK),
        "32" => Ok(BlockErase::ThirtyTwoK),
        "64" => Ok(BlockErase::SixtyFourK),
        _ => Err(GenericArgumentError::new(
            "Only valid values are 4, 32, or 64",
        )),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BlockErase {
    FourK,
    ThirtyTwoK,
    SixtyFourK,
}

impl fmt::Display for BlockErase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockErase::FourK => write!(f, "4"),
            BlockErase::ThirtyTwoK => write!(f, "32"),
            BlockErase::SixtyFourK => write!(f, "64"),
        }
    }
}
