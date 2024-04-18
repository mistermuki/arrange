#[derive(Debug, Clone, Copy)]
pub enum ArrangeError {
    WriteError,
    ReadError,
    DeviceError,
}
