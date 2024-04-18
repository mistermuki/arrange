use std::{
    ffi::{c_int, c_uchar},
    process::exit,
    thread::sleep,
    time::Duration,
};

use arrange_misc::error::ArrangeError;
use libftdi1_sys::{
    ftdi_context, ftdi_deinit, ftdi_disable_bitbang, ftdi_get_error_string, ftdi_get_latency_timer,
    ftdi_init, ftdi_interface, ftdi_mpsse_mode, ftdi_new, ftdi_read_data, ftdi_set_bitmode,
    ftdi_set_interface, ftdi_set_latency_timer, ftdi_usb_close, ftdi_usb_open,
    ftdi_usb_purge_buffers, ftdi_usb_reset, ftdi_write_data,
};
use log::{debug, error, info};

/// Mode commands
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum MPSSECommand {
    ///  Set Data bits Low Byte
    SETBLOW = 0x80,
    ///  Read Data bits Low Byte
    READBLOW = 0x81,
    ///  Set Data bits High Byte
    SETBHIGH = 0x82,
    ///  Read data bits High Byte
    READBHIGH = 0x83,
    ///  Enable loopback
    LOOPBACKEN = 0x84,
    ///  Disable loopback
    LOOPBACKDIS = 0x85,
    ///  Set clock divisor
    SETCLKDIV = 0x86,
    ///  Flush buffer fifos to the PC.
    FLUSH = 0x87,
    ///  Wait on GPIOL1 to go high.
    WAITH = 0x88,
    ///  Wait on GPIOL1 to go low.
    WAITL = 0x89,
    ///  Disable /5 div, enables 60MHz master clock
    TCKX5 = 0x8A,
    ///  Enable /5 div, backward compat to FT2232D
    TCKD5 = 0x8B,
    ///  Enable 3 phase clk, DDR I2C
    EN3PHCLK = 0x8C,
    ///  Disable 3 phase clk
    DIS3PHCLK = 0x8D,
    ///  Clock every bit, used for JTAG
    CLKN = 0x8E,
    ///  Clock every byte, used for JTAG
    CLKN8 = 0x8F,
    ///  Clock until GPIOL1 goes high
    CLKTOH = 0x94,
    ///  Clock until GPIOL1 goes low
    CLKTOL = 0x95,
    ///  Enable adaptive clocking
    ENADPTCLK = 0x96,
    ///  Disable adaptive clocking
    DISADPTCLK = 0x97,
    ///  Clock until GPIOL1 goes high, count bytes
    CLK8TOH = 0x9C,
    ///  Clock until GPIOL1 goes low, count bytes
    CLK8TOL = 0x9D,
    ///  Set IO to only drive on 0 and tristate on 1
    TRI = 0x9E,
    ///  CPUMode read short address
    CPURS = 0x90,
    ///  CPUMode read extended address
    CPURE = 0x91,
    ///  CPUMode write short address
    CPUWS = 0x92,
    ///  CPUMode write extended address
    CPUWE = 0x93,
}

/// Encapsulates all of the MPSSE (Multi-Protocol Synchronous Serial Engine) instructions used.

pub struct MPSSE {
    context: *mut ftdi_context,
    latency: c_uchar,
    open: bool,
    latency_set: bool,
}

impl MPSSE {
    const FTDI_VENDOR: c_int = 0x0403;
    const DEVICE_ID_1: c_int = 0x6010;
    const DEVICE_ID_2: c_int = 0x6014;

    ///  When set use TMS mode
    const DATA_TMS: u8 = 0x40;
    ///  When set read data (Data IN)
    const DATA_IN: u8 = 0x20;
    ///  When set write data (Data OUT)
    const DATA_OUT: u8 = 0x10;
    ///  When set input/output data LSB first.
    const DATA_LSB: u8 = 0x08;
    ///  When set receive data on negative clock edge
    const DATA_ICN: u8 = 0x04;
    ///  When set count bits not bytes
    const DATA_BITS: u8 = 0x02;
    ///  When set update data on negative clock edge
    const DATA_OCN: u8 = 0x01;

    pub fn new() -> Self {
        Self {
            context: unsafe { ftdi_new() },
            latency: b'0',
            open: false,
            latency_set: false,
        }
    }

    pub fn init(
        &mut self,
        interface: ftdi_interface,
        device_string: Option<String>,
        slow_clock: bool,
    ) -> Result<(), ArrangeError> {
        unsafe { ftdi_init(self.context) };
        unsafe { ftdi_set_interface(self.context, interface) };

        // Opening the USB connection with the FTDI device.
        match device_string {
            Some(_) => todo!("Implement Device String"),
            None => {
                let status_1 =
                    unsafe { ftdi_usb_open(self.context, MPSSE::FTDI_VENDOR, MPSSE::DEVICE_ID_1) };
                debug!(
                    "Status of ftdi_usb_open on Device ID: {:#x} is: {status_1}",
                    MPSSE::DEVICE_ID_1
                );
                let status_2 =
                    unsafe { ftdi_usb_open(self.context, MPSSE::FTDI_VENDOR, MPSSE::DEVICE_ID_2) };
                debug!(
                    "Status of ftdi_usb_open on Device ID: {:#x} is: {status_2}",
                    MPSSE::DEVICE_ID_2
                );

                if status_1 != 0 && status_2 != 0 {
                    error!(
                        "can't find iCE FTDI USB Device (vendor_id {:#x} with device ids {:#x} or {:#x})",
                        MPSSE::FTDI_VENDOR,
                        MPSSE::DEVICE_ID_1,
                        MPSSE::DEVICE_ID_2,
                    );

                    return Err(ArrangeError::DeviceError);
                }
            }
        }

        self.open = true;

        // Try to reset the FTDI Chip.
        let reset_status = unsafe { ftdi_usb_reset(self.context) };
        debug!("FTDI USB Reset Status: {reset_status}");
        if reset_status != 0 {
            error!("Failed to reset iCE FTDI USB device.\n");
            return Err(ArrangeError::DeviceError);
        }

        // Purge USB Buffers.
        let purge_status = unsafe { ftdi_usb_purge_buffers(self.context) };
        debug!("FTDI USB Buffer Purge Status: {reset_status}");
        if purge_status != 0 {
            error!("Failed to purge buffers on iCE FTDI USB device.\n");
            return Err(ArrangeError::DeviceError);
        }

        // Gets the latency.
        let latency_ptr: *mut c_uchar = &mut self.latency;
        let get_latency_status = unsafe { ftdi_get_latency_timer(self.context, latency_ptr) };
        debug!("FTDI USB Get Latency Status: {get_latency_status}");
        debug!("FTDI USB Latency Value: {:#x}", self.latency);
        if get_latency_status != 0 {
            error!("Failed to get latency timer: {:?}.", unsafe {
                ftdi_get_error_string(self.context)
            });

            return Err(ArrangeError::DeviceError);
        }

        // Sets the latency to 1 kHz polling.
        let set_latency_status = unsafe { ftdi_set_latency_timer(self.context, 1) };
        debug!("FTDI USB Set Latency Status: {set_latency_status}");
        if set_latency_status != 0 {
            error!("Failed to get latency timer: {:?}.", unsafe {
                ftdi_get_error_string(self.context)
            });
            return Err(ArrangeError::DeviceError);
        }
        self.latency_set = true;

        let set_mpsse_mode_status =
            unsafe { ftdi_set_bitmode(self.context, 0xff, ftdi_mpsse_mode::BITMODE_MPSSE.0 as u8) };
        debug!("FTDI USB Set MPSSE Mode Status: {set_mpsse_mode_status}");
        if set_mpsse_mode_status != 0 {
            error!("Failed to set MPSSE mode on iCE FTDI USB device.\n");
            return Err(ArrangeError::DeviceError);
        }

        // clock divide by 5.
        // maybe don't? could potentially be faster...
        // TODO: i should generate an echo benchmark.
        self.send_byte(MPSSECommand::TCKD5 as u8);

        if slow_clock {
            info!("Setting FTDI USB to Slow Mode: 50 kHz");
            self.send_byte(MPSSECommand::SETCLKDIV as u8);
            self.send_byte(119);
            self.send_byte(0);
        } else {
            info!("Setting FTDI USB to Normal Mode: 6 MHz");
            self.send_byte(MPSSECommand::SETCLKDIV as u8);
            self.send_byte(0);
            self.send_byte(0);
        }

        Ok(())
    }

    /// Blocks while waiting to receive a byte.
    pub fn recv_byte(&self) -> u8 {
        let mut data: u8 = 0;
        let data_ptr: *mut u8 = &mut data;
        loop {
            let read_count = unsafe { ftdi_read_data(self.context, data_ptr, 1) };
            if read_count < 0 {
                error!("Read Error!");
                self.error(2)
            }

            if read_count == 1 {
                break;
            }

            sleep(Duration::from_millis(1));
        }

        data
    }

    /// Writes a byte to the FTDI Device.
    pub fn send_byte(&self, data: u8) {
        let data_ptr: *const u8 = &data;
        let write_count = unsafe { ftdi_write_data(self.context, data_ptr, 1) };
        if write_count != 1 {
            error!(
                "Error writing byte to FTDI. Expected {} bytes to be written, only got {}",
                1, write_count
            );
            self.error(2);
        }
    }

    pub fn send_spi(&self, data: &[u8]) {
        if data.len() < 1 {
            return;
        }

        self.send_byte(MPSSE::DATA_OUT | MPSSE::DATA_OCN);
        self.send_byte((data.len() - 1) as u8);
        self.send_byte(((data.len() - 1) >> 8) as u8);

        let data_ptr: *const u8 = data.as_ptr();
        let write_count = unsafe { ftdi_write_data(self.context, data_ptr, data.len() as c_int) };
        if write_count != data.len() as i32 {
            error!(
                "Error writing data. Expected {} bytes to be written, only got {}",
                data.len(),
                write_count
            );
            self.error(2);
        }
    }

    pub fn transfer_spi(&self, data: &[u8]) -> Result<Vec<u8>, ()> {
        if data.len() < 1 {
            return Ok(vec!());
        }

        self.send_byte(MPSSE::DATA_IN | MPSSE::DATA_OUT | MPSSE::DATA_OCN);
        self.send_byte(data.len() as u8 - 1);
        self.send_byte(((data.len() - 1) >> 8) as u8);

        let data_ptr: *const u8 = data.as_ptr();
        let write_count = unsafe { ftdi_write_data(self.context, data_ptr, data.len() as c_int) };
        if write_count != data.len() as i32 {
            error!(
                "Error writing data to FTDI. Expected {} bytes to be written, only got {}",
                data.len(),
                write_count
            );
            return Err(());
        }
    
        let mut return_vec = vec!();
        while return_vec.len() < data.len() {
            return_vec.push(self.recv_byte())
        }

        Ok(return_vec)
    }

    pub fn transfer_spi_bits(&self, data: u8, n: u8) -> u8 {
        self.send_byte(MPSSE::DATA_IN | MPSSE::DATA_OUT | MPSSE::DATA_OCN | MPSSE::DATA_BITS);
        self.send_byte(n - 1);
        self.send_byte(data);

        self.recv_byte()
    }

    pub fn set_gpio(&self, gpio: u8, direction: u8) -> () {
        self.send_byte(MPSSECommand::SETBLOW as u8);
        self.send_byte(gpio);
        self.send_byte(direction);
    }

    pub fn read_low_byte(&self) -> u8 {
        self.send_byte(MPSSECommand::READBLOW as u8);
        self.recv_byte()
    }

    pub fn read_high_byte(&self) -> u8 {
        self.send_byte(MPSSECommand::READBHIGH as u8);
        self.recv_byte()
    }

    /// This closes our FTDI context.
    pub fn close(&self) -> () {
        unsafe { ftdi_set_latency_timer(self.context, self.latency) };
        unsafe { ftdi_disable_bitbang(self.context) };
        unsafe { ftdi_usb_close(self.context) };
        unsafe { ftdi_deinit(self.context) };
    }

    /// On error, we need to close down the FTDI context and exit from the program.
    pub fn error(&self, status: i32) -> ! {
        // check rx
        error!("ABORT.");
        if self.open {
            if self.latency_set {
                unsafe { ftdi_set_latency_timer(self.context, self.latency) };
            }

            unsafe { ftdi_usb_close(self.context) };
        }
        unsafe { ftdi_deinit(self.context) };
        exit(status);
    }
}
