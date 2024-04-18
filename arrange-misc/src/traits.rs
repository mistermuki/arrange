/// Generic trait for Arrange implementions.
pub trait Arrange {
    /// Creates a new Arrange instance.
    fn new() -> Self;
    /// Initializes any data that may be needed. 
    fn init(&mut self) -> Result<(), ()>;
    /// Checks if the Arrange device is present.
    fn check(&self) -> bool; 
    /// Burn a slice of bytes to the Arrange device.
    fn burn_bytes(&self, bytes: &[u8]) -> Result<(), ()>;
}   
