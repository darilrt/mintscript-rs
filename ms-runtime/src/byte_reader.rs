pub(crate) struct ByteReader<'a> {
    source: &'a Vec<u8>,
    position: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(source: &'a Vec<u8>) -> ByteReader<'a> {
        ByteReader {
            source: source,
            position: 0,
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.position < self.source.len() {
            let byte = self.source[self.position];
            self.position += 1;
            Some(byte)
        } else {
            None
        }
    }

    pub fn read_bytes(&mut self, count: usize) -> Option<Vec<u8>> {
        if self.position + count <= self.source.len() {
            let bytes = self.source[self.position..self.position + count].to_vec();
            self.position += count;
            Some(bytes)
        } else {
            None
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        let bytes = self.read_bytes(4)?;

        Some(u32::from_le_bytes([bytes[3], bytes[2], bytes[1], bytes[0]]))
    }

    pub fn read_i32(&mut self) -> Option<i32> {
        let bytes = self.read_bytes(4)?;

        Some(i32::from_le_bytes([bytes[3], bytes[2], bytes[1], bytes[0]]))
    }

    pub fn read_f32(&mut self) -> Option<f32> {
        let bytes = self.read_bytes(4)?;

        Some(f32::from_le_bytes([bytes[3], bytes[2], bytes[1], bytes[0]]))
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        let byte = self.read_byte()?;

        Some(byte != 0)
    }

    // Read a string from the source with the following format:
    // <length: u32> <string: [u8 x length]>
    pub fn read_string(&mut self) -> Option<String> {
        // Read the length of the string
        let length = self.read_u32()? as usize;

        // Read the string
        let bytes = self.read_bytes(length)?;

        // Convert the bytes to a string
        Some(String::from_utf8(bytes).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_reader_read_bytes() {
        let source = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = ByteReader::new(&source);

        assert_eq!(reader.read_byte(), Some(0x01));
        assert_eq!(reader.read_byte(), Some(0x02));
        assert_eq!(reader.read_bytes(2), Some(vec![0x03, 0x04]));
        assert_eq!(reader.read_byte(), None);
    }

    #[test]
    fn module_reader_read_bytes_out_of_bounds() {
        let source = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = ByteReader::new(&source);

        assert_eq!(reader.read_bytes(5), None);
    }

    #[test]
    fn module_reader_read_string() {
        let source = vec![0, 0, 0, 4, b't', b'e', b's', b't'];
        let mut reader = ByteReader::new(&source);

        assert_eq!(reader.read_string(), Some("test".to_string()));
    }

    #[test]
    fn module_reader_read_string_out_of_bounds() {
        let source = vec![0, 0, 0, 5, b't', b'e', b's', b't'];
        let mut reader = ByteReader::new(&source);

        assert_eq!(reader.read_string(), None);
    }

    #[test]
    fn module_reader_read_u32() {
        let source = vec![0x89, 0xAB, 0xCD, 0xEF];
        let mut reader = ByteReader::new(&source);

        assert_eq!(reader.read_u32(), Some(0x89ABCDEF));
    }

    #[test]
    fn module_reader_read_u32_out_of_bounds() {
        let source = vec![0x89, 0xAB, 0xCD];
        let mut reader = ByteReader::new(&source);

        assert_eq!(reader.read_u32(), None);
    }
}
