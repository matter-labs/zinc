pub struct Bytecode {
    cursor: usize,
    bytes: Vec<u8>,
}

impl Bytecode {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            cursor: 0,
            bytes: Vec::from(bytes),
        }
    }

    pub fn next_byte(&mut self) -> Option<u8> {
        if self.cursor >= self.bytes.len() {
            return None;
        }

        let byte = self.bytes[self.cursor];
        self.cursor += 1;
        Some(byte)
    }

    pub fn next_bytes(&mut self, len: usize) -> Option<Vec<u8>> {
        if self.cursor + len > self.bytes.len() {
            return None;
        }

        let bytes = &self.bytes[self.cursor..(self.cursor+len)];
        self.cursor += len;
        Some(Vec::from(bytes))
    }

    pub fn is_eof(&self) -> bool {
        self.cursor == self.bytes.len()
    }
}
