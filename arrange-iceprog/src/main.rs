/*
 * Allows you to program an iCE40 FPGA through a FTDI 2322.
 * (sorta) a drop in replacement for iceprog.
 */
use std::{
    fs::File,
    io::{Read, Seek},
    process::exit,
    thread::sleep,
    time::Duration,
};

use arrange::{
    prelude::*,
    FTDI::{flash::Flash, test_mode::TestMode},
};
use clap::{CommandFactory, Parser};
use log::{debug, error, info};

use crate::cli::arguments::Arguments;
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
    let args = Arguments::parse();
    if args.verbose {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }

    debug!("Arguments: {:?}", args);
    debug!("Command: {}", Arguments::command());
    debug!("File Name: {}", args.file_name);

    let file: Option<File> = {
        if args.test_mode != TestMode::NoTest {
            // We don't care about the file in test mode.
            None
        } else if args.read_mode {
            Some(
                File::options()
                    .write(true)
                    .open(&args.file_name)
                    .unwrap_or_else(|_| {
                        error!("Cannot open '{}' for writing.", args.file_name);
                        exit(1);
                    }),
            )
        } else {
            Some(
                File::options()
                    .read(true)
                    .open(&args.file_name)
                    .unwrap_or_else(|_| {
                        error!("Cannot open '{}' for reading.", args.file_name);
                        exit(1);
                    }),
            )
        }
    };

    // Create Arrange.
    let mut arrange = arrange::Arrange::new();
    eprintln!("Initializing MPSSE...");
    arrange.init().unwrap();
    let mpsse = arrange.get_mpsse();
    eprintln!("MPSSE initialized.");
    read_cdone!(mpsse);

    let flash = Flash::new(&mpsse);
    flash.release_reset();
    sleep(Duration::from_millis(100));
    eprintln!("Reset...");

    if args.test_mode != TestMode::NoTest {
        // If in test mode...
        flash.chip_deselect();
        sleep(Duration::from_millis(250));

        read_cdone!(mpsse);
        flash.reset();
        flash.power_up();
        if args.test_mode == TestMode::Quad {
            //flash.enable_quad();
            todo!("Implement QUAD SPI");
        } else {
            flash.read_id();
        }
        flash.power_down();
        flash.release_reset();
        sleep(Duration::from_millis(250));
        read_cdone!(mpsse);
    } else if args.prog_sram {
        // Programming SRAM
        todo!("Implement SRAM programming");
    } else {
        // Programming FLASH
        assert!(file.is_some());
        let mut f = file.unwrap();
        let file_size = f.metadata().unwrap().len() as usize;
        if !args.read_mode && !args.check_mode {
            if args.disable_protect {
                flash.write_enable();
                flash.disable_protection();
            }

            if !args.dont_erase {
                if args.bulk_erase {
                    flash.write_enable();
                    flash.bulk_erase();
                    flash.wait();
                } else {
                    // Erase enough for the file.
                    eprintln!("File Size: {file_size}");
                    let block_size = (args.block_erase_size as usize) << 10;
                    eprintln!("Block Size: {block_size}");
                    let block_mask = block_size - 1;
                    let begin_addr = args.address_offset & !block_mask;
                    let end_addr = (args.address_offset + file_size + block_mask) & !block_mask;

                    for addr in (begin_addr..end_addr).step_by(block_size) {
                        flash.write_enable();
                        flash.sector_erase(args.block_erase_size, addr);

                        debug!("Status after Block Erase: {}", flash.read_status());
                        flash.wait();
                    }
                }
            }

            if args.erase_blocks.is_none() {
                eprintln!("Programming...");

                let mut addr = 0;
                'chunks: loop {
                    let mut buffer: [u8; 256] = [0; 256];
                    // Read chunks out of the file.
                    let read_count = match f.read(&mut buffer) {
                        Ok(0) => break 'chunks,
                        Ok(value) => value,
                        Err(_) => {
                            error!("Unable to continue reading from file...");
                            mpsse.close();
                            exit(2);
                        }
                    };

                    info!(
                        "addr {:#06X} {}",
                        args.address_offset + addr,
                        100 * addr / file_size
                    );

                    // Write those chunks into the FLASH.
                    flash.write_enable();
                    flash.prog(args.address_offset + addr, &buffer[..read_count]);
                    flash.wait();

                    addr += read_count;
                }

                eprintln!("done.");
                f.seek(std::io::SeekFrom::Start(0)).unwrap();
            }
        }

        if !args.disable_powerdown {
            flash.power_down();
        }

        flash.release_reset();
        sleep(Duration::from_millis(250));
        read_cdone!(mpsse);
    }

    eprintln!("Bye.");
    mpsse.close();
}
