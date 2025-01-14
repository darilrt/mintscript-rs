pub(crate) struct ByteWriter<'a> {
    source: &'a mut Vec<u8>,
}

impl<'a> ByteWriter<'a> {
    pub fn new(source: &'a mut Vec<u8>) -> ByteWriter<'a> {
        ByteWriter { source: source }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.source.push(byte);
    }

    #[inline]
    pub fn write_bytes(&mut self, bytes: &Vec<u8>) {
        self.source.extend(bytes);
    }

    #[inline]
    pub fn write_u32(&mut self, value: u32) {
        self.source.extend(value.to_be_bytes().iter());
    }

    #[inline]
    pub fn write_i32(&mut self, value: i32) {
        self.source.extend(value.to_be_bytes().iter());
    }

    #[inline]
    pub fn write_f32(&mut self, value: f32) {
        self.source.extend(value.to_be_bytes().iter());
    }

    #[inline]
    pub fn write_string(&mut self, value: &str) {
        self.write_u32(value.len() as u32);
        self.source.extend(value.as_bytes());
    }
}
