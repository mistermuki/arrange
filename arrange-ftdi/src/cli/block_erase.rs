use super::error::GenericArgumentError;
use core::fmt;

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
    FourK = 4,
    ThirtyTwoK = 32,
    SixtyFourK = 64,
}

impl fmt::Display for BlockErase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self as usize).to_string())
    }
}
