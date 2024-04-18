use crate::error::ArrangeError;

/// Generic trait for Arrange implementions.
pub trait Arrange {
    /// Creates a new Arrange instance.
    fn new() -> Self;
    /// Initializes any data that may be needed. This will also check and ensure the device is
    /// present and ready to be used.
    fn init(&mut self) -> Result<(), ArrangeError>;
    /// Burn a slice of bytes to the Arrange device.
    fn burn_bytes(&self, bytes: &[u8]) -> Result<(), ArrangeError>;
    /// Sends all of the bytes to the Arrange device.
    fn send_bytes(&self, bytes:&[u8]) -> Result<(), ArrangeError>;
    /// Receives a byte from the Arrange device.
    fn recv_byte(&self) -> Result<u8, ArrangeError>; 
}   
