use std::io::{self, Write};
use std::process;

/// Extension to `Write`.
pub trait WriteExt {
    /// Write a string of bytes with a newline appended.
    fn writeln(&mut self, s: &[u8]) -> io::Result<usize>;

    /// Write a character.
    fn write_char(&mut self, c: char) -> io::Result<usize>;
}

impl<W: Write> WriteExt for W {
    fn writeln(&mut self, s: &[u8]) -> io::Result<usize> {
        let res = self.write(s);
        match self.write(b"\n") {
            Ok(n) => res.map(|x| x + n),
            e => e,
        }
    }

    fn write_char(&mut self, c: char) -> io::Result<usize> {
        let mut utf8 = [0; 4];
        match c.encode_utf8(&mut utf8) {
            Some(len) => self.write(&utf8[0..len]),
            None => unreachable!(),     // A buffer of length four is large enough to encode any char
        }
    }
}

pub fn fail<'a>(s: &'a str, stderr: &mut io::Stderr) -> ! {
    let mut stderr = stderr.lock();

    let _ = stderr.write(b"error: ");
    let _ = stderr.write(s.as_bytes());
    let _ = stderr.write(b"\n");
    let _ = stderr.flush();
    process::exit(1);
}
