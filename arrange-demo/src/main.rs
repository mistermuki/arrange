use std::time::Instant;

use arrange::prelude::*;
use log::info;

fn main() {
    env_logger::init();
    let bitstream = include_bytes!("hw/build/bitstream.bin");
    info!("Bitstream Byte Count: {}", bitstream.len());

    // Create and initalize Arrange.
    let mut arrange = arrange::Arrange::new();
    match arrange.init() {
        Ok(_) => {
            // If we have a valid Arrange device.
            let now = Instant::now();
            arrange.burn(bitstream).unwrap();
            let elapsed = now.elapsed();

            // ~126 ms if you have the same bitstream.
            // ~1.34 seconds if you have a different one.
            //
            // could be sped up by doing incremental flashing?
            // so only rewrite when you find a difference.
            println!("Burning Time: {:?}", elapsed);
        }

        Err(_) => {
            // If we don't have a valid Arrange device.
            println!("No Arrange Device detected.");
        }
    }

    println!("Hello from Arrange!");
}
