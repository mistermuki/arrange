use crate::cli::arguments::Arguments;
use arrange_ftdi::ftdi::mpsse::MPSSE;
use clap::{CommandFactory, Parser};
use log::debug;
mod cli;

pub fn main() {
    env_logger::init();
    let args = Arguments::parse();
    debug!("Arguments: {:?}", args);
    debug!("Command: {}", Arguments::command());
    debug!("File Name: {}", args.file_name);

    // We need to first open the given file.
    //let f: File = File::open(args.file_name).unwrap();
    
    // Initalize USB Connection to FT2232H
    let mut mpsse = MPSSE::new();

    println!("Initializing MPSSE...");
    mpsse.init(args.ftdi_chip_interface_select, args.device_string, args.slow_clock);
    println!("MPSSE initialized.");
}
