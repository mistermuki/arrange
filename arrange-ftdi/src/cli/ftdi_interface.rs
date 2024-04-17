use libftdi1_sys::ftdi_interface;

use super::error::GenericArgumentError;

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
