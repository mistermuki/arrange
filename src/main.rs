mod arrange;

fn main() {
    let bitstream = include_bytes!("./hw/build/arrange.bin");
    println!("Bitstream: {bitstream:?}");
}
