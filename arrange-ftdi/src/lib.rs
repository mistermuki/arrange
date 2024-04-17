pub mod cli;
pub mod ftdi;

// TODD:
//
// add generic bindings that are shared with potentially other arrange-* crates.
// this should probably be done in an arrange-traits crate that everything else uses.
//
// there should basically be a method for:
//      - reset
//      - program (from bytes)
//      - program (from file)
//      - send_raw_cmd?
//
// eventually:
//      - send_byte (to FPGA)
//      - recv_byte (from FPGA)
//      - BufReader interface?
//      - BufWriter interface?
