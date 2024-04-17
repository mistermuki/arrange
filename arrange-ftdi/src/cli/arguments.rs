use super::{block_erase::{block_erase_parser, BlockErase}, ftdi_interface::ftdi_interface_parser};
use clap::Parser;
use libftdi1_sys::ftdi_interface;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg()]
    pub file_name: String,

    #[arg(short)]
    pub device_string: Option<String>,
    #[arg(short = 'i', default_value_t = BlockErase::SixtyFourK, help = "select erase block size",value_parser = block_erase_parser)]
    pub block_erase_size: BlockErase, 
    #[arg(
        short = 'I',
        default_value = "A",
        help = "connect to the specified interface on the FTDI chip",
        value_parser = ftdi_interface_parser 
    )]
    pub ftdi_chip_interface_select: ftdi_interface, 
    #[arg(short = 'r', default_value_t = false, help = "reads 256 bytes from flash")]
    pub read_mode: bool,
    #[arg(short = 'R', help = "reads N bytes from flash")]
    pub read_n_bytes: Option<usize>,
    #[arg(short = 'e')]
    pub erase_blocks: Option<usize>,
    // Do we want to support k and M endings?
    #[arg(short = 'o', default_value_t = 0)]
    pub address_offset: usize,

    #[arg(short = 'c', default_value_t = false)]
    pub check_mode: bool,

    #[arg(short = 'b', default_value_t = false)]
    pub bulk_erase: bool,

    #[arg(short = 'n', default_value_t = false)]
    pub dont_erase: bool,

    #[arg(short = 'S', default_value_t = false)]
    pub prog_sram: bool,

    // test_mode = 1
    #[arg(short = 't', default_value_t = false)]
    pub test_mode: bool,

    // test_mode = 2
    #[arg(short = 'T', default_value_t = false)]
    pub q_test_mode: bool,

    /// Farts!
    #[arg(short = 'v', default_value_t = false)]
    pub verbose: bool,

    #[arg(short = 's', default_value_t = false)]
    pub slow_clock: bool,

    #[arg(short = 'p', default_value_t = false)]
    pub disable_protect: bool,

    #[arg(short = 'X', default_value_t = false)]
    pub disable_verify: bool,

    #[arg(short = 'k', default_value_t = false)]
    pub disable_powerdown: bool,
}
