use super::error::GenericArgumentError;
use arrange::FTDI::block_erase::BlockErase;
use arrange::FTDI::test_mode::TestMode;
use libftdi1_sys::ftdi_interface;

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

pub fn ftdi_interface_parser(arg: &str) -> Result<ftdi_interface, GenericArgumentError> {
    match arg {
        "A" => Ok(ftdi_interface::INTERFACE_A),
        "B" => Ok(ftdi_interface::INTERFACE_B),
        "C" => Ok(ftdi_interface::INTERFACE_C),
        "D" => Ok(ftdi_interface::INTERFACE_D),
        _ => Err(GenericArgumentError::new(
            "Only valid values are A, B, C or D",
        )),
    }
}
