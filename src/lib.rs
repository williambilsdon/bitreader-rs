pub struct Bitreader<'a> {
    buffer: &'a [u8],
    position: u8,
    length: u8,
}

impl<'a> Bitreader<'a> {
    pub fn new(bytes: &'a [u8]) -> Bitreader {
        Bitreader { 
            buffer: bytes, 
            position: 0, 
            length: bytes.len() as u8 * 8
        }
    }

    fn read_bits(&self, size: u8) -> u8 {
        let mut result: u8 = 0;
        let pos = self.position;
        let end_pos = self.position + size;

        for i in pos..end_pos {
            let index = (i / 8) as usize;
            let byte = self.buffer[index]; // 10
            let shift = 7 - (i % 8); // 0
            let bit = (byte >> shift) & 1; // 0001 0100 >> 0 = 0001 0100 & 1 = 0000 0000 

            result = (result << 1) | bit; // 0000 1010 << 1 = 0001 0100 | 0000 0000  = 0001 0100 
            println!("{}", result)
        }

        result
    }

    // pub fn read_string(&self, size: u8) -> [u8] {
    //     let end_pos = self.position + size as u64;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_bits() {
        let input: &[u8] = &[10];
        let bitreader = Bitreader::new(input);

        let result = bitreader.read_bits(8);
        assert_eq!(result, 10)
    }
}
