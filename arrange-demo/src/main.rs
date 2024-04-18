use std::time::Instant;

use arrange::prelude::*;
use log::info;
use text_io::read;

fn main() {
    let bitstream = include_bytes!("hw/build/bitstream.bin");
    env_logger::init();
    info!("Bitstream Byte Count: {}", bitstream.len());

    print!("Should we burn our bitstream onto the FPGA? [Y/N]: ");

    // Create and initalize Arrange.
    let mut arrange = arrange::Arrange::new();
    arrange.init().unwrap();

    let answer: String = read!("{}\n");

    if answer.to_lowercase().starts_with('y') {
        let now = Instant::now();
        arrange.burn_bytes(bitstream).unwrap();
        let elapsed = now.elapsed();

        println!("Burning Time: {:?}", elapsed);
    }

    println!("Hello from Arrange!");
}
