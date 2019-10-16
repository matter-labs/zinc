//!
//! The transpiler writer.
//!

#[derive(Default)]
pub struct Writer {
    buffer: String,
    offset: usize,
}

impl Writer {
    const SIZE_TAB: usize = 4;

    pub fn get(&mut self) -> String {
        let result = self.buffer.clone();
        self.buffer.clear();
        result
    }

    pub fn write_line(&mut self, line: String) {
        self.write_offset();
        self.buffer.push_str(&line);
        self.write_new_line();
    }

    pub fn write_lines(&mut self, lines: Vec<String>) {
        for line in lines.into_iter() {
            self.write_line(line);
        }
    }

    pub fn shift_forward(&mut self) {
        self.offset += Self::SIZE_TAB;
    }

    pub fn shift_backward(&mut self) {
        if self.offset >= Self::SIZE_TAB {
            self.offset -= Self::SIZE_TAB;
        } else {
            self.offset = 0;
        }
    }

    fn write_offset(&mut self) {
        self.buffer.push_str(&" ".repeat(self.offset));
    }

    fn write_new_line(&mut self) {
        #[cfg(windows)]
        self.buffer.push_str("\r\n");
        #[cfg(not(windows))]
        self.buffer.push('\n');
    }
}
