use std::{fs::File, process::exit, thread::sleep, time::Duration};

use crate::cli::{arguments::Arguments, test_mode::TestMode};
use arrange_ftdi::ftdi::{flash::Flash, mpsse::MPSSE};
use clap::{CommandFactory, Parser};
use log::{debug, error, info};
mod cli;

macro_rules! read_cdone {
    ($mpsse: expr) => {{
        if $mpsse.read_low_byte() & 0x40 != 0 {
            info!("cdone: high");
        } else {
            info!("cdone: low");
        }
    }};
}

pub fn main() {
    env_logger::init();
    let args = Arguments::parse();
    debug!("Arguments: {:?}", args);
    debug!("Command: {}", Arguments::command());
    debug!("File Name: {}", args.file_name);

    let _f: Option<File> = {
        if args.test_mode != TestMode::NoTest {
            // We don't care about the file in test mode.
            None
        } else if args.read_mode {
            Some(
                File::options()
                    .write(true)
                    .open(&args.file_name)
                    .unwrap_or_else(|_| {
                        error!("Cannot open {} for writing.", args.file_name);
                        exit(1);
                    }),
            )
        } else {
            Some(
                File::options()
                    .read(true)
                    .open(&args.file_name)
                    .unwrap_or_else(|_| {
                        error!("Cannot open {} for reading.", args.file_name);
                        exit(1);
                    }),
            )
        }
    };

    // Create MPSSE object
    let mut mpsse = MPSSE::new();

    println!("Initializing MPSSE...");
    mpsse.init(
        args.ftdi_chip_interface_select,
        args.device_string,
        args.slow_clock,
    );
    println!("MPSSE initialized.");
    read_cdone!(mpsse);

    let flash = Flash::new(&mpsse);
    flash.release_reset();
    sleep(Duration::from_millis(100));
    info!("Reset...");

    if args.test_mode != TestMode::NoTest {
        flash.chip_deselect();
        sleep(Duration::from_millis(250));

        read_cdone!(mpsse);
        flash.reset();
        flash.power_up();
        if args.test_mode == TestMode::Quad {
            //flash.enable_quad();
        } else {
            flash.read_id();
        }
        flash.power_down();
        flash.release_reset();
        sleep(Duration::from_millis(250));
        read_cdone!(mpsse);
    } else {
        todo!("Implement");
    }

    eprintln!("Bye.");
    mpsse.close();
}
