use std::io;
use std::io::{Error, ErrorKind, Read, Write};

/// A struct which implements [std::io::Read] and [std::io::Write] by just always reporting an
/// error.  LoadLetter violates the social contracts of [Read] and [Write], and as a result your
/// program may have undesirable behaviour if you try to use a LoadLetter.
#[derive(Debug)]
pub struct LoadLetter<'a> {
    kind: ErrorKind,
    error: &'a str,
}

impl<'a> LoadLetter<'a> {
    pub fn new(kind: ErrorKind) -> Self {
        let error = "";
        Self { kind, error }
    }

    pub fn new_msg(kind: ErrorKind, error: &'a str) -> Self {
        Self { kind, error }
    }

    pub fn error(&self) -> Error {
        Error::new(self.kind, self.error)
    }
}

impl Read for LoadLetter<'_> {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(Error::new(self.kind, self.error))
    }
}

impl Write for LoadLetter<'_> {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(Error::new(self.kind, self.error))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(Error::new(self.kind, self.error))
    }
}

impl Default for LoadLetter<'_> {
    fn default() -> Self {
        let kind = ErrorKind::Other;
        let error = "PC Load Letter";
        LoadLetter { kind, error }
    }
}

#[cfg(test)]
#[test]
fn create() {
    let ll = LoadLetter::new_msg(ErrorKind::Other, "Kaiju attack");
    let err = ll.error();
    assert_eq!(ErrorKind::Other, err.kind());
}
