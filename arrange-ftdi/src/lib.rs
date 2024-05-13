use arrange_misc::{error::ArrangeError, traits::Arrange};
use ftdi::{flash::Flash, mpsse::MPSSE};
use libftdi1_sys::ftdi_interface;
use log::{debug, info};

use crate::ftdi::block_erase::BlockErase;

pub mod ftdi;

pub struct ArrangeFTDI<'a> {
    flash_interface: MPSSE<'a>,
    comm_interface: MPSSE<'a>,
}

impl<'a> ArrangeFTDI<'a> {
    pub fn get_mpsse(&self, programming: bool) -> &MPSSE {
        if programming {
            &self.flash_interface
        } else {
            &self.comm_interface
        }
    }

    pub fn get_mpsse_mut(&mut self, programming: bool) -> &'a mut MPSSE {
        if programming {
            &mut self.flash_interface
        } else {
            &mut self.comm_interface
        }
    }

    pub fn get_flash(&'a mut self, programming: bool) -> Flash {
        Flash::new(if programming { &mut self.flash_interface } else { &mut self.comm_interface })
    }
}

impl<'a> Arrange for ArrangeFTDI<'a> {
    fn new() -> Self {
        Self {
            flash_interface: MPSSE::new(),
            comm_interface: MPSSE::new(),
        }
    }

    fn init(&mut self) -> Result<(), ArrangeError> {
        // We can only program over Interface A.
        // We can only communicate over Interface B.
        self.flash_interface.init(ftdi_interface::INTERFACE_A, None, false)?;
        self.comm_interface.init(ftdi_interface::INTERFACE_B, None, false)
    }

    fn burn(&mut self, bytes: &[u8]) -> Result<(), ArrangeError> {
        let mut flash = Flash::new(&mut self.flash_interface);

        // Reset.
        flash.release_reset()?;
        info!("Reset...");

        // Erase enough for the bytes.
        let bytes_size = bytes.len();
        info!("Bytes Size: {bytes_size}");

        // Read first to ensure we aren't writing the same stream back.
        info!("Checking...");
        let mut addr = 0;
        let mut same = true;
        for chunk in bytes.chunks(256) {
            debug!("addr {:#06X} {}", addr, 100 * addr / bytes_size);
            let read = flash.read(addr, chunk.len())?;

            if chunk != read {
                info!("Difference. Let's program!");
                same = false;
                break;
            }

            addr += chunk.len();
        }

        if same {
            info!("Skipping programming..");
        } else {
            // We're going to default to a Block Size of 64.
            let block_size = (BlockErase::SixtyFourK as usize) << 10;
            info!("Block Size: {block_size}");
            let block_mask = block_size - 1;

            // We are going to assume an offset of 0.
            let begin_addr = 0;
            let end_addr = (bytes_size + block_mask) & !block_mask;

            info!("Erasing...");
            for addr in (begin_addr..end_addr).step_by(block_size) {
                flash.write_enable()?;
                flash.sector_erase(BlockErase::SixtyFourK, addr)?;
                debug!("Status after Block Erase: {}", flash.read_status()?);
                flash.wait()?;
            }

            info!("Programming...");
            let mut addr = 0;
            for chunk in bytes.chunks(256) {
                debug!("addr {:#06X} {}", addr, 100 * addr / bytes_size);

                // Write those chunks into the FLASH.
                flash.write_enable()?;
                flash.prog(addr, chunk)?;
                flash.wait()?;

                addr += chunk.len();
            }

            info!("Verifying...");
            addr = 0;
            for chunk in bytes.chunks(256) {
                debug!("addr {:#06X} {}", addr, 100 * addr / bytes_size);
                let read = flash.read(addr, chunk.len())?;

                if chunk != read {
                    debug!("Found difference between flash and bytes!");
                    return Err(ArrangeError::WriteError);
                }

                addr += chunk.len();
            }

            info!("Verified, OK!");
        }

        flash.release_reset()?;
        Ok(())
    }

    fn read(&self) -> Result<Vec<u8>, ArrangeError> {
        todo!("Implement at some point");
    }

    fn send(&mut self, bytes: &[u8]) -> Result<(), ArrangeError> {
        // We are programming on interface A.
        // When we are sending and recieving, we want to do it over interface B.
        Ok(())
    }

    fn recv(&mut self, length: usize) -> Result<Vec<u8>, ArrangeError> {
        todo!()
    }
}
