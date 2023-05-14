use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum BitreadError{
    BufferExceeded,
    ParseToStringError
}

impl fmt::Display for BitreadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitreadError::BufferExceeded => write!(f, "Unable to read bits, read size exceeds remaining buffer length."),
            BitreadError::ParseToStringError => write!(f, "Unable to parse bytes to String type.")
        }
    }
}
