use arrange_misc::{error::ArrangeError, traits::Arrange};
use ftdi::mpsse::MPSSE;
use libftdi1_sys::ftdi_interface;

pub mod ftdi;

pub struct ArrangeFTDI {
    mpsse: MPSSE,
}

impl ArrangeFTDI {
    pub fn get_mpsse(&self) -> &MPSSE {
        &self.mpsse
    }
}

impl Arrange for ArrangeFTDI {
    fn new() -> Self {
        Self {
            mpsse: MPSSE::new(),
        }
    }

    fn init(&mut self) -> Result<(), ArrangeError> {
        self.mpsse.init(ftdi_interface::INTERFACE_A, None, false)
    }

    fn check(&self) -> bool {
        false
    }

    fn burn_bytes(&self, bytes: &[u8]) -> Result<(), ArrangeError> {
        Err(ArrangeError::DeviceError)
    }
}
