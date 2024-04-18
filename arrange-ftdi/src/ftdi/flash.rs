use std::{thread::sleep, time::Duration};

use log::{debug, error, info};

use super::{block_erase::BlockErase, mpsse::MPSSE};

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

        let data: [u8; 5] = [FlashCommand::JEDECID as u8; 5];
        debug!("Read Flash ID...");
        self.chip_select();

        let jedec = self.mpsse.transfer_spi(&data[..5]).unwrap();

        let e_dev = {
            if jedec[4] == 0xff {
                error!(
                "Extended Device String Length is 0xFF, this is likely a read error. Ignoring..."
            );
                panic!()
            } else if jedec[4] != 0 {
                // We should read out the rest of the bytes...
                debug!("Getting Extended Device String of length: {}", jedec[4]);
                self.mpsse.transfer_spi(&vec![0; jedec[4] as usize]).unwrap()
            } else {
                panic!()
            }
        };

        debug!("Flash MFG ID: {:#x}", jedec[1]);
        debug!("Flash Dev ID #1: {:#x}", jedec[2]);
        debug!("Flash Dev ID #2: {:#x}", jedec[3]);
        debug!("Flash Extended Dev String Length: {:#X}", jedec[4]);

        let mut flash_id: String = String::new();
        flash_id.push_str(&format!("{:#02X} ", jedec[1]));
        flash_id.push_str(&format!("{:#02X} ", jedec[2]));
        flash_id.push_str(&format!("{:#02X} ", jedec[3]));
        for d in e_dev[1..e_dev.len()].into_iter() {
            flash_id.push_str(&format!("{:#02X} ", d));
        }

        info!("Flash ID: {flash_id}");
    }

    pub fn reset(&self) -> () {
        let cmd: [u8; 8] = [0xff; 8];

        self.chip_select();
        self.mpsse.transfer_spi(&cmd).unwrap();
        self.chip_deselect();

        self.chip_select();
        self.mpsse.transfer_spi_bits(0xff, 2);
        self.chip_deselect();
    }

    pub fn power_up(&self) -> () {
        let cmd: [u8; 1] = [FlashCommand::RPD as u8];
        self.chip_select();
        self.mpsse.transfer_spi(&cmd).unwrap();
        self.chip_deselect();
    }

    pub fn power_down(&self) -> () {
        let cmd: [u8; 1] = [FlashCommand::PD as u8];
        self.chip_select();
        self.mpsse.transfer_spi(&cmd).unwrap();
        self.chip_deselect();
    }

    pub fn read_status(&self) -> u8 {
        let cmd: [u8; 2] = [FlashCommand::RSR1 as u8; 2];
        self.chip_select();
        let response = self.mpsse.transfer_spi(&cmd).unwrap();
        self.chip_deselect();

        debug!("SR1: {:#02X}", response[1]);
        debug!(
            "SPRL: {}",
            if response[1] & (1 << 7) == 0 {
                "Unlocked"
            } else {
                "Locked"
            }
        );
        debug!(
            "SPM: {}",
            if response[1] & (1 << 6) == 0 {
                "Byte/Page Prog Mode"
            } else {
                "Sequential Prog Mode"
            }
        );
        debug!(
            "EPE: {}",
            if response[1] & (1 << 5) == 0 {
                "Erase/Prog Success"
            } else {
                "Erase/Prog Error"
            }
        );
        debug!(
            "SPM: {}",
            if response[1] & (1 << 4) == 0 {
                "!WP Asserted"
            } else {
                "!WP Deasserted"
            }
        );
        debug!(
            "SWP: {}",
            match response[1] >> 2 & 0x3 {
                0 => "All sectors unprotected",
                1 => "Some sectors protected",
                2 => "Reserved (xxxx 10xx)",
                3 => "All sectors protected",
                _ => "Unexpected value, ignoring...",
            }
        );
        debug!(
            "WEL: {}",
            if response[1] & (1 << 1) == 0 {
                "Not Write Enabled"
            } else {
                "Write Enabled"
            }
        );
        debug!(
            "RDY: {}",
            if response[1] & (1 << 0) == 0 {
                "Ready"
            } else {
                "Busy"
            }
        );

        response[1]
    }

    pub fn write_enable(&self) -> () {
        debug!("Status before enable: {}", self.read_status());
        debug!("Enabling Write...");

        let cmd: [u8; 1] = [FlashCommand::WE as u8];
        self.chip_select();
        self.mpsse.transfer_spi(&cmd).unwrap();
        self.chip_deselect();
    }

    pub fn bulk_erase(&self) -> () {
        info!("Bulk Erase...");

        let cmd: [u8; 1] = [FlashCommand::CE as u8];
        self.chip_select();
        self.mpsse.transfer_spi(&cmd).unwrap();
        self.chip_deselect();
    }

    pub fn sector_erase(&self, be: BlockErase, addr: usize) -> () {
        info!("Erase {be}kB sector at {:#06X}", addr);

        let command: [u8; 4] = match be {
            BlockErase::FourK => [
                FlashCommand::SE as u8,
                (addr >> 16) as u8,
                (addr >> 8) as u8,
                addr as u8,
            ],
            BlockErase::ThirtyTwoK => [
                FlashCommand::BE32 as u8,
                (addr >> 16) as u8,
                (addr >> 8) as u8,
                addr as u8,
            ],
            BlockErase::SixtyFourK => [
                FlashCommand::BE64 as u8,
                (addr >> 16) as u8,
                (addr >> 8) as u8,
                addr as u8,
            ],
        };

        self.chip_select();
        self.mpsse.send_spi(&command);
        self.chip_deselect();
    }

    pub fn prog(&self, addr: usize, data: &[u8]) -> () {
        debug!("prog {:#06X} +{:#03X}", addr, data.len());

        let cmd: [u8; 4] = [
            FlashCommand::PP as u8,
            (addr >> 16) as u8,
            (addr >> 8) as u8,
            addr as u8,
        ];

        self.chip_select();
        self.mpsse.send_spi(&cmd);
        self.mpsse.send_spi(data);
        self.chip_deselect();

        let mut debug_str = String::new();

        for i in 0..data.len() {
            debug_str.push_str(&format!(
                "{:#02x}{}",
                data[i],
                if i == data.len() - 1 || i % 32 == 31 {
                    '\n'
                } else {
                    ' '
                }
            ));
        }
        debug!("\n{}", debug_str.trim_end_matches('\n'));
    }

    pub fn wait(&self) -> () {
        debug!("Waiting...");

        let mut count = 0;

        loop {
            let cmd: [u8; 2] = [FlashCommand::RSR1 as u8; 2];
            self.chip_select();
            let response = self.mpsse.transfer_spi(&cmd).unwrap();
            self.chip_deselect();

            if response[1] & 0x01 == 0 {
                if count < 2 {
                    count += 1;
                    debug!("read: {count}");
                } else {
                    debug!("R: {count}");
                    break;
                }
            } else {
                count = 0;
            }

            sleep(Duration::from_millis(1));
        }
    }

    pub fn disable_protection(&self) -> () {
        info!("Disable Flash Protection...");

        let cmd: [u8; 2] = [FlashCommand::WSR1 as u8, 0];
        self.chip_select();
        self.mpsse.transfer_spi(&cmd).unwrap();
        self.chip_deselect();
        self.wait();

        let cmd2: [u8; 2] = [FlashCommand::RSR1 as u8, 0];
        self.chip_select();
        let response = self.mpsse.transfer_spi(&cmd2).unwrap();
        self.chip_deselect();

        if response[1] != 0 {
            error!(
                "Failed to disable protection, SR now equal to {:#02x} (expected 0x00)",
                response[1]
            );
        }
    }
}
