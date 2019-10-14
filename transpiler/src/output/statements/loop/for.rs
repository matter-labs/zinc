//!
//! Transpiler output for loop.
//!

pub struct Output {
    pub start: String,
    pub end: String,
}

impl Output {
    pub fn output(index: String, range_start: usize, range_end: usize, is_inclusive: bool) -> Self {
        let operator = if is_inclusive { "..=" } else { ".." };
        let range = if range_end >= range_start {
            format!("{0}{1}{2}", range_start, operator, range_end)
        } else {
            format!("({0}{1}{2}).rev()", range_end, operator, range_start)
        };

        let start = format!("for {0}_index in {1} {{", index, range,);
        let end = "}".to_owned();

        Self { start, end }
    }
}
