use core::fmt;
use std::fmt::Error;

pub struct Bitreader<'a> {
    buffer: &'a [u8],
    position: u64,
    length: u64,
}

impl<'a> Bitreader<'a> {
    pub fn new(bytes: &'a [u8]) -> Bitreader {
        Bitreader { 
            buffer: bytes, 
            position: 0, 
            length: bytes.len() as u64 * 8
        }
    }

    fn read_bits(&mut self) -> Result {
        let mut value: u8 = 0;
        let start_pos = self.position;
        let end_pos = start_pos + 8;

        if (end_pos > self.length) {
            BitReadError()
        }

        println!("start_pos : {}\nend_pos : {}", start_pos, end_pos);

        for i in start_pos..end_pos{
            let index = (i / 8) as usize;
            let byte = self.buffer[index]; // 10
            let shift = 7 - (i % 8); // 0
            let bit = (byte >> shift) & 1; // 0001 0100 >> 0 = 0001 0100 & 1 = 0000 0000 

            value = (value << 1) | bit; // 0000 1010 << 1 = 0001 0100 | 0000 0000  = 0001 0100 
        }

        self.position = end_pos;

        Ok(value)
    }

}

type Result = std::result::Result<u8, BitReadError>;

#[derive(Debug, Clone, PartialEq)]
struct BitReadError;

impl fmt::Display for BitReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to read bits")
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn read_bits_from_single_item() {
        let input: &[u8] = &[10];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_bits().unwrap();
        assert_eq!(result, 10)
    }

    #[test]
    fn read_bits_from_multiple_items() {
        let input: &[u8] = &[10, 20, 30];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_bits().unwrap();
        println!("{}", result);
        assert_eq!(result, 10);

        let result = bitreader.read_bits().unwrap();
        println!("{}", result);
        assert_eq!(result, 20);

        let result = bitreader.read_bits().unwrap();
        println!("{}", result);
        assert_eq!(result, 30)
    }

    #[test]
    fn read_bits_err_if_size_too_large() {
        let input: &[u8] = &[10];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_bits().err().unwrap();
        assert_eq!(result, BitReadError)
    }
}
