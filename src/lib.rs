//! # Comp IO
//!
//! `comp_io` is a collection of utilities centered around the `Reader` struct
//! to make competitive programming easier to write

use std::io::{self, Read};

/// Reads data from stdin in an optimized manner
///
/// Limitations: doesn't skip whitespace. Assumes that input data is sanitized (each number is separated by exactly 1 character)
/// This allows for faster reading of data, because in most competitive programming scenarios, the input data is already provided
/// in such a way
///
/// # Example:
///
/// ```no_run
/// let mut reader = comp_io::Reader::new();
///
/// // Read an i32:
/// let a: i32 = reader.next_i32().unwrap();
///
/// // Read a pair of i32s:
/// let (b, c): (i32, i32) = reader.next_pair().unwrap();
///
/// // Read an f64
/// let d: f64 = reader.next_f64().unwrap();
/// ```
pub struct Reader {
    buffer: Vec<u8>,
    index: usize,
    len: usize,
}
//        ___      _________________
//       /  .\    /                 \
//      /  =__|  <  Walrus Approved.|
// hjm /    ||    \_________________/
impl Iterator for Reader {
    type Item = u8;

    #[cfg(not(test))]
    fn next(&mut self) -> Option<Self::Item> {
        // If at end of buffer
        if self.index >= self.len {
            if self.len < 400_000 {
                return None;
            }
            // Try to read from stdin
            self.buffer.clear(); // necessary?
            self.len = io::stdin()
                .lock()
                .take(400_000)
                .read_to_end(&mut self.buffer)
                .ok()?;
            self.index = 0;
        }
        let n = self.buffer[self.index];
        self.index += 1;
        Some(n)
    }

    #[cfg(test)]
    fn next(&mut self) -> Option<Self::Item> {
        // If at end of buffer
        if self.index >= self.len {
            return None;
        }
        let n = self.buffer[self.index];
        self.index += 1;
        Some(n)
    }
}

impl Reader {
    /// Instantiates a new reader
    ///
    /// # Example:
    ///
    /// ```
    /// let mut reader = comp_io::Reader::new();
    /// ```
    pub fn new() -> Self {
        Reader {
            buffer: Vec::<u8>::with_capacity(400_000),
            index: usize::MAX,
            len: usize::MAX,
        }
    }

    /// Useful for testing reader without requiring access to stdin
    ///
    /// Note: Best with `cargo test`, otherwise still needs stdin
    ///
    /// # Example:
    ///
    /// ```
    /// let mut reader = comp_io::Reader::from_str("12 43\n-42");
    /// ```
    pub fn from_str(input: &str) -> Self {
        Reader {
            buffer: input.as_bytes().to_vec(),
            index: 0,
            len: input.len(),
        }
    }

    fn read_i32(&mut self) -> Option<(i32, i32)> {
        // let (mut r, mut val, neg) = (0, 48, self.next()? == b'-');
        // self.index -= if neg {0} else {1};

        let mut r = 0;
        let (mut val, neg) = match self.next()? {
            v @ b'0'..=b'9' => (v as i32, false), // could also move to the end with no ifs, don't know which is better
            b'-' => (48, true),
            b'+' => (48, false),
            _ => return None, // Unexpected character
        };

        while val >= b'0' as i32 && val <= b'9' as i32 {
            r = r * 10 + val - 48;

            val = match self.next() {
                Some(a) => a as i32,
                None => break,
            };
        }
        Some((if neg { -r } else { r }, val))
    }

    /// Reads the next u32 from stdin
    pub fn next_u32(&mut self) -> Option<u32> {
        Some(self.read_i32()?.0.unsigned_abs())
    }

    /// Reads the next usize from stdin
    pub fn next_usize(&mut self) -> Option<usize> {
        Some(self.read_i32()?.0.unsigned_abs() as usize)
    }

    /// Reads the next i32 from stdin
    pub fn next_i32(&mut self) -> Option<i32> {
        Some(self.read_i32()?.0)
    }

    /// Reads the next char from stdin
    pub fn next_char(&mut self) -> Option<char> {
        Some(self.next()? as char)
    }

    /// Reads the next pair of i32s from stdin
    pub fn next_pair(&mut self) -> Option<(i32, i32)> {
        Some((self.read_i32()?.0, self.read_i32()?.0))
    }

    /// Reads the next f64 from stdin
    pub fn next_f64(&mut self) -> Option<f64> {
        let (base, latest) = self.read_i32()?;
        let base: f64 = base as f64;
        if latest != b'.' as i32 { // number doesn't have a period
            return Some(base);
        }
        let (mut dec, mut val, mut ten) = (0.0, 48, 1.0);
        while val >= b'0' && val <= b'9' {
            dec += ((val - b'0') as f64) * ten;
            ten *= 0.1;
            val = match self.next() {
                Some(a) => a,
                _ => break,
            };
        }
        Some(base + dec.copysign(base))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_from_str() {
        let reader = Reader::from_str("-4.1");

        assert_eq!(reader.buffer, vec![45, 52, 46, 49]);
        assert_eq!(reader.index, 0);
        assert_eq!(reader.len, 4);
    }

    #[test]
    fn test_next_f64() {
        let mut reader = Reader::from_str("-4 45 -754.3 32. 45");
        assert_eq!(reader.next_f64().unwrap(), -4.);
        assert_eq!(reader.next_f64().unwrap(), 45.);
        assert_eq!(reader.next_f64().unwrap(), -754.3);
        assert_eq!(reader.next_f64().unwrap(), 32.);
        assert_eq!(reader.next_f64().unwrap(), 45.);
    }

    #[test]
    fn test_next_char() {
        let mut reader = Reader::from_str("ab cd");
        assert_eq!(reader.next_char().unwrap(), 'a');
        assert_eq!(reader.next_char().unwrap(), 'b');
        assert_eq!(reader.next_char().unwrap(), ' ');
        assert_eq!(reader.next_char().unwrap(), 'c');
        assert_eq!(reader.next_char().unwrap(), 'd');
    }

    #[test]
    fn test_next_pair() {
        let mut reader = Reader::from_str("23 32\n12 -34 57 97\n-12 3");
        assert_eq!(reader.next_pair().unwrap(), (23, 32));
        assert_eq!(reader.next_pair().unwrap(), (12, -34));
        assert_eq!(reader.next_pair().unwrap(), (57, 97));
        assert_eq!(reader.next_pair().unwrap(), (-12, 3));
    }

    #[test]
    fn test_next_usize() {
        let mut reader = Reader::from_str("23 32\n12\n34");
        assert_eq!(reader.next_usize().unwrap(), 23);
        assert_eq!(reader.next_usize().unwrap(), 32);
        assert_eq!(reader.next_usize().unwrap(), 12);
        assert_eq!(reader.next_usize().unwrap(), 34);
    }

    #[test]
    fn test_long_float() {
        let mut reader = Reader::from_str("4.323580432456786");
        assert_eq!(reader.next_f64().unwrap(), 4.323580432456786);
    }
}