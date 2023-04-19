use std::str;

mod errors;

type Result<T> = std::result::Result<T, errors::BitReadBufferExceeded>;

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

    fn read_bits(&mut self, size: u8) -> Result<u8> {
        let mut value: u8 = 0;
        let start_pos = self.position;
        let end_pos = start_pos + size as u64;

        if end_pos > self.length {
            return Err(errors::BitReadBufferExceeded)
        }

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

    fn read_string(&mut self, byte_size: u8) -> Result<&str> {
        let mut bytes: Vec<u8> = vec![];
        for bit_size in 0..byte_size * 8 {
            let byte = match self.read_bits(8) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            
            bytes.push(byte)
        }

        match str::from_utf8(bytes.as_slice()) {
            Ok(v) => Ok(v.clone()),
            Err(e) => return Err(errors::BitReadBufferExceeded)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_bits_from_single_item() {
        let input: &[u8] = &[10];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_bits(8).unwrap();
        assert_eq!(result, 10)
    }

    #[test]
    fn read_bits_from_multiple_items() {
        let input: &[u8] = &[10, 20, 30];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_bits(8).unwrap();
        assert_eq!(result, 10);

        let result = bitreader.read_bits(8).unwrap();
        assert_eq!(result, 20);

        let result = bitreader.read_bits(8).unwrap();
        assert_eq!(result, 30)
    }

    #[test]
    fn read_bits_err_if_size_too_large() {
        let input: &[u8] = &[10];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_bits(16);
        assert_eq!(result, Err(errors::BitReadBufferExceeded))
    }
}
