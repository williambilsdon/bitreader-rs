mod errors;

type Result<T> = std::result::Result<T, errors::BitreadError>;

pub struct Bitreader<'a> {
    buffer: &'a [u8],
    position: u64,
    pub length: u64,
}

impl<'a> Bitreader<'a> {
    pub fn new(bytes: &'a [u8]) -> Bitreader {
        Bitreader { 
            buffer: bytes, 
            position: 0, 
            length: bytes.len() as u64 * 8
        }
    }

    // making result a u64 is more elegant than using an enum for this. 
    fn read_bits(&mut self, size: u8) -> Result<u64> {
        let mut value: u64 = 0;
        let start_pos = self.position;
        let end_pos = start_pos + size as u64;

        if end_pos > self.length {
            return Err(errors::BitreadError::BufferExceeded)
        }

        for i in start_pos..end_pos{
            let index = (i / 8) as usize;
            let byte = self.buffer[index]; // 10
            let shift = 7 - (i % 8); // 0
            let bit = (byte >> shift) as u64 & 1; // 0001 0100 >> 0 = 0001 0100 & 1 = 0000 0000 

            value = (value << 1) | bit; // 0000 1010 << 1 = 0001 0100 | 0000 0000  = 0001 0100 
        }

        self.position = end_pos;

        Ok(value)
    }

    pub fn read_string(&mut self, byte_size: u8) -> Result<String> {
        let mut bytes: Vec<u8> = vec![];
        for _ in 0..byte_size {
            let byte = self.read_bits(8)?;
            
            bytes.push(byte as u8)
        }

        match String::from_utf8(bytes) {
            Ok(v) => Ok(v),
            Err(_) => return Err(errors::BitreadError::ParseToStringError)
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let value = self.read_bits(8)?;
        Ok(value as u8)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let value = self.read_bits(16)?;
        Ok(value as u16)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let value = self.read_bits(32)?;
        Ok(value as u32)
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        let value = self.read_bits(64)?;
        Ok(value as u64)
    }

    pub fn skip_bits(&mut self, num_bits: u8) -> Result<()> {
        let new_pos = self.position + (num_bits as u64);
        if new_pos > self.length {
            return Err(errors::BitreadError::BufferExceeded)
        }

        self.position = new_pos;
        
        Ok(())
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        let value = f32::from_bits(self.read_u32()?);
        Ok(value)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_bits_from_single_item() {
        let input: &[u8] = &[0b00001010];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_bits(8).unwrap();
        assert_eq!(result, 10)
    }

    #[test]
    fn read_bits_from_multiple_items() {
        let input: &[u8] = &[0b00001010, 0b00010100, 0b00011110];
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
        let input: &[u8] = &[0b00001010];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_bits(16);
        assert_eq!(result, Err(errors::BitreadError::BufferExceeded))
    }

    #[test]
    fn read_string() {
        let input = &[0b01001000, 0b01100101, 0b01101100, 0b01101100,
            0b01101111, 0b00101100, 0b00100000, 0b01010111, 0b01101111,
            0b01110010, 0b01101100, 0b01100100, 0b00100001 ];
        let mut bitreader = Bitreader::new(input);
        
        let result = bitreader.read_string(13);
        let expected = Ok(String::from("Hello, World!"));
        assert_eq!(result, expected)
    }

    #[test]
    fn read_u8() {
        let input = &[0b11111111];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_u8();
        let expected: Result<u8> = Ok(255);
        assert_eq!(result, expected) 
    }

    #[test]
    fn read_u16() {
        let input = &[0b11111111, 0b11111111];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_u16();
        let expected: Result<u16> = Ok(65535);
        assert_eq!(result, expected) 
    }

    #[test]
    fn read_u32() {
        let input = &[0b01111111, 0b11111111, 0b11111111, 0b11111111];
        let mut bitreader = Bitreader::new(input);
        println!("{}", bitreader.length);
        let result = bitreader.read_u32();
        let expected: Result<u32> = Ok(2147483647);
        assert_eq!(result, expected) 
    }

    #[test]
    fn read_u64() {
        let input = &[0b01111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_u64();
        let expected: Result<u64> = Ok(9223372036854775807);
        assert_eq!(result, expected) 
    }

    #[test]
    fn skip_bits() {
        let input = &[0b11111111, 0b11111111, 0b11111111, 0b11111111];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.skip_bits(16);
        assert_eq!(result, Ok(()));
        let expected: u64 = 16;
        assert_eq!(bitreader.position, expected)
    }

    #[test]
    fn read_f32() {
        let input = &[0b01000000, 0b01100001, 0b01000111, 0b10101110];
        let mut bitreader = Bitreader::new(input);

        let result = bitreader.read_f32();
        let expected: Result<f32> = Ok(3.52);
        assert_eq!(result, expected) 
    }
    
    #[test]
    #[ignore = "f64 isn't required right now so not implemented"]
    fn read_f64() {
        todo!()
    }


}
