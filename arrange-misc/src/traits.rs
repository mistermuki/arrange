use crate::error::ArrangeError;

/// Generic trait for Arrange implementions.
pub trait Arrange {
    /// Creates a new Arrange instance.
    fn new() -> Self;
    /// Initializes any data that may be needed. This will also check and ensure the device is
    /// present and ready to be used.
    fn init(&mut self) -> Result<(), ArrangeError>;
    /// Burn a slice of bytes to the Arrange device.
    fn burn(&mut self, bytes: &[u8]) -> Result<(), ArrangeError>;
    /// Read out the bitstream on the Arrange device.
    fn read(&self) -> Result<Vec<u8>, ArrangeError>; 
    /// Sends out the given slice of bytes.
    fn send(&mut self, bytes: &[u8]) -> Result<(), ArrangeError>;
    /// Recieves a set of bytes.
    fn recv(&mut self, length: usize) -> Result<Vec<u8>, ArrangeError>;
}   
