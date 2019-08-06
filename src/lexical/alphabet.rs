//!
//! The lexical alphabet.
//!

pub struct Alphabet;

impl Alphabet {
    pub fn contains(byte: u8) -> bool {
        (b'\t' <= byte && byte <= b'\r')
            || (b' ' <= byte && byte <= b'"')
            || (b'%' <= byte && byte <= b'>')
            || (b'A' <= byte && byte <= b'_')
            || (b'a' <= byte && byte <= b'}')
    }
}
