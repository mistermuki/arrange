use arrange_misc::traits::Arrange;
use ftdi::mpsse::MPSSE;
use libftdi1_sys::ftdi_interface;

pub mod ftdi;

pub struct ArrangeFTDI {
    mpsse: MPSSE,
}

impl Arrange for ArrangeFTDI {
    fn new() -> Self {
        Self {
            mpsse: MPSSE::new(),
        }
    }

    fn init(&mut self) -> Result<(), ()> {
        self.mpsse.init(ftdi_interface::INTERFACE_A, None, false);
        Ok(())
    }

    fn check(&self) -> bool {
        false
    }

    fn burn_bytes(&self, bytes: &[u8]) -> Result<(), ()> {
        Err(())
    }
}
