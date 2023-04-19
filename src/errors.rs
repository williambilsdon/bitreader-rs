use core::fmt;

pub type Result = std::result::Result<u8, BitReadBufferExceeded>;

#[derive(Debug, Clone, PartialEq)]
pub struct BitReadBufferExceeded;

impl fmt::Display for BitReadBufferExceeded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to read bits, read size exceeds remaining buffer length.")
    }
}