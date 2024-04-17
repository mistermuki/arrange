use log::{debug, error};

use super::mpsse::MPSSE;

pub enum FlashCommand {
    ///  Write Enable
    WE = 0x06,
    ///  Volatile SR Write Enable
    SRWE = 0x50,
    ///  Write Disable
    WD = 0x04,
    ///  Release Power-Down, returns Device ID
    RPD = 0xAB,
    ///   Read Manufacturer/Device ID
    MFGID = 0x90,
    ///  Read JEDEC ID
    JEDECID = 0x9F,
    ///  Read Unique ID
    UID = 0x4B,
    ///  Read Data
    RD = 0x03,
    ///  Fast Read
    FR = 0x0B,
    ///  Page Program
    PP = 0x02,
    ///  Sector Erase 4kb
    SE = 0x20,
    ///  Block Erase 32kb
    BE32 = 0x52,
    ///  Block Erase 64kb
    BE64 = 0xD8,
    ///  Chip Erase
    CE = 0xC7,
    ///  Read Status Register 1
    RSR1 = 0x05,
    ///  Write Status Register 1
    WSR1 = 0x01,
    ///  Read Status Register 2
    RSR2 = 0x35,
    ///  Write Status Register 2
    WSR2 = 0x31,
    ///  Read Status Register 3
    RSR3 = 0x15,
    ///  Write Status Register 3
    WSR3 = 0x11,
    ///  Read SFDP Register
    RSFDP = 0x5A,
    ///  Erase Security Register
    ESR = 0x44,
    ///  Program Security Register
    PSR = 0x42,
    ///  Read Security Register
    RSR = 0x48,
    ///  Global Block Lock
    GBL = 0x7E,
    ///  Global Block Unlock
    GBU = 0x98,
    ///  Read Block Lock
    RBL = 0x3D,
    ///  Read Sector Protection Registers (adesto)
    RPR = 0x3C,
    ///  Individual Block Lock
    IBL = 0x36,
    ///  Individual Block Unlock
    IBU = 0x39,
    ///  Erase / Program Suspend
    EPS = 0x75,
    ///  Erase / Program Resume
    EPR = 0x7A,
    ///  Power-down
    PD = 0xB9,
    ///  Enter QPI mode
    QPI = 0x38,
    ///  Enable Reset
    ERESET = 0x66,
    ///  Reset Device
    RESET = 0x99,
}

pub struct Flash<'a> {
    mpsse: &'a MPSSE,
}

impl<'a> Flash<'a> {
    pub fn new(mpsse: &'a MPSSE) -> Self {
        Self { mpsse }
    }

    fn set_cs_creset(&self, cs_b: u32, creset_b: u32) -> () {
        let gpio: u8 = 0;
        let mut direction: u8 = 0x03;

        if cs_b == 0 {
            direction |= 0x10;
        }

        if creset_b == 0 {
            direction |= 0x80;
        }

        self.mpsse.set_gpio(gpio, direction);
    }

    pub fn release_reset(&self) -> () {
        self.set_cs_creset(1, 1);
    }

    pub fn chip_select(&self) -> () {
        self.set_cs_creset(0, 0);
    }

    pub fn chip_deselect(&self) -> () {
        self.set_cs_creset(1, 0);
    }

    pub fn read_id(&self) -> () {
        /* JEDEC ID structure:
         * Byte No. | Data Type
         * ---------+----------
         *        0 | FC_JEDECID Request Command
         *        1 | MFG ID
         *        2 | Dev ID 1
         *        3 | Dev ID 2
         *        4 | Ext Dev Str Len
         */

        let mut data: Vec<u8> = vec![FlashCommand::JEDECID as u8; 5];
        let mut len = 5;
        debug!("Read Flash ID...");
        self.chip_select();
        self.mpsse.transfer_spi(&mut data[..5]);

        if data[4] == 0xff {
            error!(
                "Extended Device String Length is 0xFF, this is likely a read error. Ignoring..."
            );
        } else if data[4] != 0 {
            // We should read out the rest of the bytes...
            len += data[4] as usize;
            data.extend(vec![0; data[4] as usize]);
            debug!("Extending flash data to be of size: {}", data.len());
            self.mpsse.transfer_spi(&mut data[4..len]);
        }

        debug!("Flash MFG ID: {:#x}", data[1]);
        debug!("Flash Dev ID #1: {:#x}", data[2]);
        debug!("Flash Dev ID #2: {:#x}", data[3]);
        debug!("Flash Extended Dev String Length: {:#X}", data[4]);

        print!("Flash ID: ");
        for d in data[1..len - 1].into_iter() {
            print!("{:#02X} ", d);
        }
        println!();
    }

    pub fn reset(&self) -> () {
        let mut data: [u8; 8] = [0xff; 8];

        self.chip_select();
        self.mpsse.transfer_spi(&mut data);
        self.chip_deselect();

        self.chip_select();
        self.mpsse.transfer_spi_bits(0xff, 2);
        self.chip_deselect();
    }

    pub fn power_up(&self) -> () {
        let mut data: [u8; 1] = [FlashCommand::RPD as u8];
        self.chip_select();
        self.mpsse.transfer_spi(&mut data);
        self.chip_deselect();
    }

    pub fn power_down(&self) -> () {
        let mut data: [u8; 1] = [FlashCommand::PD as u8];
        self.chip_select();
        self.mpsse.transfer_spi(&mut data);
        self.chip_deselect();
    }
}
